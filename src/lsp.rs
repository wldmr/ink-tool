use std::{collections::HashMap, fs::File, io::Write, path::PathBuf};

use tokio::sync::Mutex;
use tower_lsp::{jsonrpc::Result, lsp_types::*, LanguageServer};
use tree::InkAddressCollector;
use type_sitter_lib::Node;

use crate::ink_syntax::Visitor;

mod tree;

#[derive(Default, Debug)]
pub struct ServerState {
    documents: HashMap<Url, tree_sitter::Tree>,
}

pub struct LspBackend {
    client: tower_lsp::Client,
    log: Mutex<File>,
    parser: Mutex<tree_sitter::Parser>,
    state: Mutex<ServerState>,
}

impl LspBackend {
    pub fn new(client: tower_lsp::Client) -> Self {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let log_path = PathBuf::from_iter([&home, ".cache", "ink-tool", "lsp.log"].into_iter());
        if !log_path.exists() {
            eprintln!("{} doesn't exist yet.", log_path.to_string_lossy());
            let cache_dir = &log_path
                .parent()
                .expect("this thing we just created definitely has a parent directory");
            if !cache_dir.exists() {
                eprintln!("{} doesn't exist yet either.", cache_dir.to_string_lossy());
                std::fs::create_dir_all(cache_dir).expect("logging directory should be creatable");
            }
        }
        let log_file = File::options()
            .create(true)
            .append(true)
            .open(log_path)
            .expect("log file should be created");
        let log = Mutex::new(log_file);
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter::Language::from(tree_sitter_ink::LANGUAGE))
            .expect("If this doesn't work, we've got bigger problems.");
        Self {
            client,
            log,
            parser: Mutex::new(parser),
            state: Mutex::new(ServerState::default()),
        }
    }
}

impl LspBackend {
    async fn read_files(&self) {
        for f in walkdir::WalkDir::new(".") {
            if let Ok(item) = f {
                if item.path().extension().is_some_and(|it| it == "ink") {
                    let string = match std::fs::read(item.path()) {
                        Ok(it) => it,
                        Err(_) => continue,
                    };
                    let url =
                        Url::from_file_path(item.path()).expect("file should have a valid url");
                    let tree = self
                        .parser
                        .lock()
                        .await
                        .parse(string, None)
                        .expect("parsing should work");
                    self.state.lock().await.documents.insert(url, tree);
                }
            }
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for LspBackend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        let _ = writeln!(self.log.lock().await, "init: {:#?}", params);

        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                text_document_sync: Some(TextDocumentSyncCapability::Options(
                    TextDocumentSyncOptions {
                        open_close: Some(true),
                        change: Some(TextDocumentSyncKind::FULL),
                        will_save: None,
                        will_save_wait_until: None,
                        save: None,
                    },
                )),
                document_symbol_provider: Some(OneOf::Left(true)),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "server initialized!")
            .await;
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.client
            .show_message(
                MessageType::INFO,
                format!("file opened: {}", params.text_document.uri),
            )
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        self.client
            .show_message(
                MessageType::INFO,
                format!("file changed: {}", params.text_document.uri),
            )
            .await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.client
            .show_message(
                MessageType::INFO,
                format!("file changed: {}", params.text_document.uri),
            )
            .await;
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        self.client
            .show_message(
                MessageType::INFO,
                format!("document symbol: {}", params.text_document.uri),
            )
            .await;

        let src = std::fs::read_to_string(
            params
                .text_document
                .uri
                .to_file_path()
                .expect("this should resolve")
                .as_path(),
        )
        .expect("Why wouldn't reading a file work");

        let mut parser = self.parser.lock().await;
        let doc = parser
            .parse(src.as_bytes(), None)
            .expect("document should be in there");
        let mut addresses = InkAddressCollector::default();
        let mut cursor = doc.root_node().walk();
        addresses.walk(&mut cursor);
        let symbols: Vec<_> = addresses
            .defs
            .iter()
            .map(|it| match it {
                tree::AddressDef::Knot { knot } => DocumentSymbol {
                    name: text(&src, &knot.name()).to_owned(),
                    detail: None,
                    kind: SymbolKind::CLASS,
                    tags: None,
                    deprecated: None,
                    range: range(knot),
                    selection_range: range(&knot.name()),
                    children: None,
                },
                tree::AddressDef::Stitch { stitch, knot } => DocumentSymbol {
                    name: match knot {
                        Some(knot) => format!(
                            "{}.{}",
                            text(&src, &knot.name()),
                            text(&src, &stitch.name())
                        ),
                        None => text(&src, &stitch.name()).to_owned(),
                    },
                    detail: None,
                    kind: SymbolKind::METHOD,
                    tags: None,
                    deprecated: None,
                    range: range(stitch),
                    selection_range: range(&stitch.name()),
                    children: None,
                },
                tree::AddressDef::Label {
                    label,
                    knot,
                    stitch,
                } => DocumentSymbol {
                    name: match (knot, stitch) {
                        (Some(knot), Some(stitch)) => format!(
                            "{}.{}.{}",
                            text(&src, &knot.name()),
                            text(&src, &stitch.name()),
                            text(&src, &label.name())
                        ),
                        (Some(knot), None) => {
                            format!("{}.{}", text(&src, &knot.name()), text(&src, &label.name()))
                        }
                        (None, Some(stitch)) => format!(
                            "{}.{}",
                            text(&src, &stitch.name()),
                            text(&src, &label.name())
                        ),
                        (None, None) => text(&src, &label.name()).to_owned(),
                    },
                    detail: None,
                    kind: SymbolKind::METHOD,
                    tags: None,
                    deprecated: None,
                    range: range(label),
                    selection_range: range(&label.name()),
                    children: None,
                },
            })
            .collect();
        Ok(Some(DocumentSymbolResponse::Nested(symbols)))
    }

    async fn hover(&self, _: HoverParams) -> Result<Option<Hover>> {
        Ok(Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "you are indeed **hovering**".to_string(),
            }),
            range: None,
        }))
    }
}

fn text<'s, 'a, T: Node<'a>>(txt: &'s str, node: &T) -> &'s str {
    &txt[node.byte_range()]
}

fn range<'a, T: Node<'a>>(node: &T) -> Range {
    Range {
        start: Position {
            line: node.start_position().row as u32,
            character: node.start_position().column as u32,
        },
        end: Position {
            line: node.end_position().row as u32,
            character: node.end_position().column as u32,
        },
    }
}
