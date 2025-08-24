use super::{state::DocumentNotFound, SharedState};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
    Router,
};
use itertools::Itertools;
use lsp_types::Uri;
use std::{future::Future, str::FromStr};
use tap::Pipe;

pub fn start<F>(state: SharedState, shutdown: F) -> Result<(), std::io::Error>
where
    F: Future<Output = ()> + Send + 'static,
{
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            // build our application with a single route
            let app = Router::new()
                .route("/", get(root))
                .route("/workspace-symbols", get(workspace_symbols))
                .route("/links", get(links))
                .route("/file/{*pth}", get(file))
                .with_state(state);

            let mut port = 1701u32;
            let listener = loop {
                match tokio::net::TcpListener::bind(format!("localhost:{port}")).await {
                    Ok(it) => break it,
                    Err(err) if err.kind() == std::io::ErrorKind::AddrInUse => {
                        port += 1;
                        continue;
                    }
                    Err(err) => panic!("{err}"),
                }
            };
            log::info!(
                "lsp-tool introspection server running on <http://{}>",
                listener.local_addr().unwrap()
            );
            axum::serve(listener, app)
                .with_graceful_shutdown(shutdown)
                .await
        })
}

async fn root(State(state): State<SharedState>) -> impl IntoResponse {
    let state = state.lock().expect("I want this lock!");
    let common_prefix = state.common_file_prefix();
    html::root::Html::builder()
        .body(|body| {
            body.heading_1(|h| h.text("Ink-Tool"));
            body.paragraph(|p| {
                p.text(format!(
                    "directory: {}",
                    std::env::current_dir().unwrap().to_string_lossy()
                ))
            });
            body.unordered_list(|menu| {
                menu.list_item(|li| {
                    li.anchor(|a| a.href("workspace-symbols").text("Workspace Symbols"))
                });
                menu.list_item(|li| li.anchor(|a| a.href("links").text("Links")));
                menu.list_item(|li| {
                    li.text("Files");
                    li.unordered_list(|ul| {
                        for uri in state.uris() {
                            ul.list_item(|li| {
                                let path = uri.path().segments().join("/");
                                li.anchor(|a| {
                                    a.href(format!("/file/{path}",))
                                        .text(path.replace(&common_prefix, ""))
                                })
                            });
                        }
                        ul
                    })
                });
                menu
            })
        })
        .build()
        .to_string()
        .replace(&common_prefix, "")
        .pipe(axum::response::Html)
}

async fn workspace_symbols(State(state): State<SharedState>) -> impl IntoResponse {
    let mut state = state.lock().expect("I want this lock!");

    let mut syms = html::text_content::UnorderedList::builder();
    for sym in state.workspace_symbols(String::new()) {
        syms.list_item(|li| li.push(sym.convert()));
    }

    let common_prefix = state.common_file_prefix();
    html::root::Html::builder()
        .body(|body| body.push(syms.build()))
        .build()
        .to_string()
        .replace(&common_prefix, "") // a bit of a kludge, but it'll do
        .pipe(axum::response::Html)
}

async fn links(State(state): State<SharedState>) -> impl IntoResponse {
    let state = state.lock().expect("I want this lock!");
    let links: Vec<(
        String,
        String,
        tree_sitter::Node<'_>,
        Vec<(String, Vec<(String, tree_sitter::Node<'_>)>)>,
    )> = todo!("rewriting links, please don't use");

    let common_prefix = state.common_file_prefix();

    let mut body = html::root::Body::builder();
    body.heading_1(|h1| h1.text("Links"));
    body.unordered_list(|def_ul| {
        for (def_path, def_text, def_node, refs) in links {
            def_ul.list_item(|def_item| {
                def_item.text(format!(
                    "{def_path}:{}:{}: {def_text}",
                    def_node.start_position().row + 1,
                    def_node.start_position().column + 1
                ));
                def_item.unordered_list(|references| {
                    for (ref_path, refs) in refs {
                        references.list_item(|ref_item| {
                            ref_item.text(ref_path);
                            ref_item.unordered_list(|usages_per_uri| {
                                for (ref_text, ref_node) in refs {
                                    usages_per_uri.list_item(|refitem| {
                                        refitem.text(format!(
                                            "{}:{}: {ref_text}",
                                            ref_node.start_position().row + 1,
                                            ref_node.start_position().column + 1
                                        ))
                                    });
                                }
                                usages_per_uri
                            })
                        });
                    }
                    references
                });
                def_item
            });
        }
        def_ul
    });

    let result = html::root::Html::builder()
        .push(body.build())
        .build()
        .to_string();
    result.pipe(axum::response::Html)
}

async fn file(
    Path(path): Path<std::path::PathBuf>,
    State(state): State<SharedState>,
) -> axum::response::Result<String> {
    let state = state.lock().expect("I want this lock!");
    let uri = lsp_types::Uri::from_str(&format!("file:///{}", path.to_string_lossy())).map_err(
        |err| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                err.to_string(),
            )
        },
    )?;
    let text = state.text(uri)?.clone();
    Ok(text)
}

impl IntoResponse for DocumentNotFound {
    fn into_response(self) -> axum::response::Response {
        (axum::http::StatusCode::NOT_FOUND, self.to_string()).into_response()
    }
}

trait To<T> {
    fn convert(self) -> T;
}

impl To<html::text_content::DescriptionList> for lsp_types::WorkspaceSymbol {
    fn convert(self) -> html::text_content::DescriptionList {
        html::text_content::DescriptionList::builder()
            .description_term(|term| term.text("name"))
            .description_details(|deets| deets.text(self.name))
            .description_term(|term| term.text("kind"))
            .description_details(|deets| deets.text(format!("{:?}", self.kind)))
            .description_term(|term| term.text("location"))
            .description_details(|deets| deets.text(fmt_location(&self.location)))
            .build()
    }
}

fn fmt_location(
    loc: &lsp_types::OneOf<lsp_types::Location, lsp_types::WorkspaceLocation>,
) -> String {
    match loc {
        lsp_types::OneOf::Left(loc) => format!(
            "{}:{}:{}–{}:{}",
            loc.uri.path().as_str(),
            loc.range.start.line + 1,
            loc.range.start.character + 1,
            loc.range.end.line + 1,
            loc.range.end.character + 1,
        ),
        lsp_types::OneOf::Right(loc) => format!("{}", loc.uri.as_str()),
    }
}
