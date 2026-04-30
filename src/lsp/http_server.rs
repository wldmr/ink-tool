use crate::lsp::{
    salsa::{file_globals, global_names, globals, ink_inventory, local_resolutions, Name},
    InkGetters,
};

use super::{state::DocumentNotFound, SharedState};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
    Router,
};
use ink_document::ids::NodeId;
use mini_milc::Db as _;
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
                .route("/file/{*pth}", get(file::<html::root::Html>))
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
                menu.list_item(|li| {
                    li.text("Files");
                    li.unordered_list(|ul| {
                        for uri in state.uris() {
                            ul.list_item(|li| {
                                let path = uri.path().as_str();
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
    let state = state.lock().expect("I want this lock!");

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

async fn file<R>(
    Path(path): Path<std::path::PathBuf>,
    State(state): State<SharedState>,
) -> Result<axum::response::Html<String>, (axum::http::StatusCode, String)> {
    let state = state.lock().expect("I want this lock!");
    let prefix = &*state.db.common_path_prefix();
    let uri = lsp_types::Uri::from_str(&format!("file://{prefix}{}", path.to_string_lossy()))
        .map_err(|err| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                err.to_string(),
            )
        })?;
    let docid = state
        .db
        .doc_ids()
        .get_id(&uri)
        .ok_or_else(|| DocumentNotFound(uri.clone()))?;

    let html = html::root::Html::builder()
        .body(|body| {
            body.heading_1(|h| h.text(state.db.short_path(docid).clone()));

            let q = ink_inventory { docid };
            let it = &*state.db.get(q);
            body.heading_2(|h| {
                h.text(format!(
                    "Inventory ({}/{})",
                    state.db.verified_at(q).unwrap(),
                    state.db.changed_at(q).unwrap()
                ))
            });
            body.preformatted_text(|p| p.text(format!("{it:#?}")));

            let q = local_resolutions { docid };
            let it = &*state.db.get(q);
            body.heading_2(|h| {
                h.text(format!(
                    "Resolutions ({}/{})",
                    state.db.verified_at(q).unwrap(),
                    state.db.changed_at(q).unwrap()
                ))
            });
            body.preformatted_text(|p| p.text(format!("{it:#?}")));

            let q = file_globals { docid };
            let it = &*state.db.get(q);
            body.heading_2(|h| {
                h.text(format!(
                    "File Globals ({}/{})",
                    state.db.verified_at(q).unwrap(),
                    state.db.changed_at(q).unwrap()
                ))
            });
            body.preformatted_text(|p| p.text(format!("{it:#?}")));

            for story in state.db.stories_of(docid).iter().copied() {
                let q = globals { story };
                let it = &*state.db.get(q);
                body.heading_2(|h| {
                    h.text(format!(
                        "Story Globals ({story:?}) ({}/{})",
                        state.db.verified_at(q).unwrap(),
                        state.db.changed_at(q).unwrap()
                    ))
                });
                body.preformatted_text(|p| p.text(format!("{it:#?}")));

                let q = global_names { story };
                let it = &*state.db.get(q);
                body.heading_2(|h| {
                    h.text(format!(
                        "Story Global Namess ({story:?}) ({}/{})",
                        state.db.verified_at(q).unwrap(),
                        state.db.changed_at(q).unwrap()
                    ))
                });
                body.preformatted_text(|p| p.text(format!("{it:#?}")));
            }

            body.heading_2(|h| h.text("Text"));
            body.preformatted_text(|pre| {
                pre.class("ink").text(state.db.document(docid).full_text())
            })
        })
        .build();

    let mut text = html.to_string();

    let names = state.db.node_text(docid);
    let locs = state.db.node_locations(docid);

    for id in locs.left_values().copied() {
        let nid = NodeId::from(id);
        let id_text = format!("{}", usize::from(nid));
        let id_name = Name::from(&id_text);
        let name = names.get(&id).unwrap_or(&id_name);
        let loc = locs
            .get_by_left(&id)
            .map(|it| format!("{it}"))
            .unwrap_or_else(|| format!("?"));
        let id = format!("{:x}", usize::from(nid));
        let id = &id[6..];
        text = text.replace(
            id_name.as_str(),
            &format!("<span>{id} <b>{name}</b> {loc}</span>"),
        );
    }

    Ok(axum::response::Html(text))
}

impl From<DocumentNotFound> for (axum::http::StatusCode, String) {
    fn from(value: DocumentNotFound) -> Self {
        (axum::http::StatusCode::NOT_FOUND, value.to_string())
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
