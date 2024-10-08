use clap::error::Result;
use lsp_types::OneOf;
use lsp_types::{InitializeParams, ServerCapabilities};

use lsp_server::{
    Connection, ExtractError, Message, ReqQueue, Request, RequestId, Response, ResponseError,
};
use request::{DocumentSymbolRequest, HoverRequest};

use std::collections::HashMap;

use lsp_types::*;
use type_sitter_lib::Node;

mod tree;

pub struct LspState {
    parser: tree_sitter::Parser,
    documents: HashMap<Uri, tree_sitter::Tree>,
}

impl LspState {
    pub fn new() -> Self {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_ink::LANGUAGE.into())
            .expect("If this fails, we can't recover.");
        Self {
            parser,
            documents: HashMap::new(),
        }
    }
}

use crate::AppResult;

// *** Config Area: Register Server behaviors here ***

fn server_capabilities() -> ServerCapabilities {
    ServerCapabilities {
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        document_symbol_provider: Some(OneOf::Left(true)),
        ..Default::default()
    }
}

static HANDLERS: &'static [RequestHandlerFn] = &[
    HoverRequest::handle_request,
    DocumentSymbolRequest::handle_request,
];

enum RequestHandlerResult {
    Success(RequestId, serde_json::Value),
    Failure(RequestId, ResponseError),
    NotInterested(Request),
}

type RequestHandlerFn = fn(Request, &mut LspState) -> RequestHandlerResult;

pub fn run_lsp() -> AppResult<()> {
    // Note that  we must have our logging only write out to stderr.
    eprintln!("starting generic LSP server");

    // Create the transport. Includes the stdio (stdin and stdout) versions but this could
    // also be implemented to use sockets or HTTP.
    let (connection, io_threads) = Connection::stdio();

    // Run the server and wait for the two threads to end (typically by trigger LSP Exit event).
    let server_capabilities = serde_json::to_value(&server_capabilities()).unwrap();
    let initialization_params = match connection.initialize(server_capabilities) {
        Ok(it) => it,
        Err(e) => {
            if e.channel_is_disconnected() {
                io_threads.join()?;
            }
            return Err(e.into());
        }
    };
    let _params: InitializeParams = serde_json::from_value(initialization_params).unwrap();
    let mut state = LspState::new();

    eprintln!("starting example main loop");
    for msg in &connection.receiver {
        eprintln!("got msg: {msg:?}");
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    return Ok(());
                }
                eprintln!("got request: {req:?}");
                match handle_request(req, &mut state).into() {
                    Ok(response) => connection.sender.send(Message::Response(response))?,
                    Err(req) => eprintln!("unhandled request {req:?}"),
                }
            }
            Message::Response(resp) => {
                eprintln!("got response: {resp:?}");
            }
            Message::Notification(not) => {
                eprintln!("got notification: {not:?}");
            }
        }
    }
    io_threads.join()?;

    // Shut down gracefully.
    eprintln!("shutting down server");
    Ok(())
}

fn handle_request(mut request: Request, state: &mut LspState) -> Result<Response, Request> {
    for handler in HANDLERS {
        use RequestHandlerResult::*;
        match handler(request, state) {
            NotInterested(it) => request = it, // let the next handler try
            Success(id, succ) => {
                return Ok(Response {
                    id,
                    result: Some(succ),
                    error: None,
                })
            }
            Failure(id, fail) => {
                return Ok(Response {
                    id,
                    result: None,
                    error: Some(fail),
                })
            }
        }
    }
    Err(request)
}

trait RequestHandler: lsp_types::request::Request {
    fn execute(params: Self::Params, state: &mut LspState) -> Self::Result;

    fn handle_request(req: Request, state: &mut LspState) -> RequestHandlerResult {
        match req.extract(Self::METHOD) {
            Ok((id, params)) => {
                // eprintln!("got gotoDefinition request #{id}: {params:?}");
                let result = Self::execute(params, state);
                let result = serde_json::to_value(&result).unwrap();
                RequestHandlerResult::Success(id, result)
            }
            Err(err @ ExtractError::JsonError { .. }) => panic!("{err:?}"), // maybe we should skip malformed json gracefully?
            Err(ExtractError::MethodMismatch(req)) => RequestHandlerResult::NotInterested(req),
        }
    }
}

impl RequestHandler for HoverRequest {
    fn execute(_params: Self::Params, _state: &mut LspState) -> Self::Result {
        Some(Hover {
            contents: HoverContents::Scalar(MarkedString::String(
                "You are indeed hovering".to_owned(),
            )),
            range: None,
        })
    }
}

impl RequestHandler for DocumentSymbolRequest {
    fn execute(params: Self::Params, _state: &mut LspState) -> Self::Result {
        let info = SymbolInformation {
            name: "Boo".to_owned(),
            kind: SymbolKind::FIELD,
            tags: None,
            deprecated: None,
            location: Location {
                uri: params.text_document.uri,
                range: Range::new(Position::new(0, 0), Position::new(0, 0)),
            },
            container_name: None,
        };
        Some(DocumentSymbolResponse::Flat(vec![info]))
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
