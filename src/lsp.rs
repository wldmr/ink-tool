use lsp_types::OneOf;
use lsp_types::{InitializeParams, ServerCapabilities};

use lsp_server::{
    Connection, ExtractError, Message, Notification, Request, RequestId, Response, ResponseError,
};
use notification::Notification as _;
use tree_sitter::{InputEdit, Point};

use std::collections::HashMap;

use lsp_types::*;
use type_sitter_lib::Node;

use crate::AppResult;

mod tree;

pub struct Document {
    tree: tree_sitter::Tree,
    text: String,
}

impl Document {
    pub fn new(parser: &mut tree_sitter::Parser) -> Self {
        let text = String::new();
        let tree = parser.parse(&text, None).expect("no reason to fail");
        Self { tree, text }
    }

    fn byte_for(&self, pos: impl Into<lsp_types::Position>) -> Option<usize> {
        let Position {
            mut line,
            mut character,
        } = pos.into();
        // Exceptionally stupid way to do it. TODO: Make more smart!
        let mut prev_idx = 0;
        for (idx, byte) in self.text.char_indices() {
            let width = idx - prev_idx;
            // eprintln!("{idx} - {line}:{character}");
            prev_idx = idx;
            if line > 0 {
                if byte == '\n' {
                    line -= 1
                }
            } else {
                if character == 0 {
                    return Some(idx);
                } else {
                    character -= width as u32;
                }
            }
        }

        None
    }
}

pub struct LspState {
    parser: tree_sitter::Parser,
    documents: HashMap<Uri, Document>,
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

// *** Config Area: Register Server behaviors here ***

fn server_capabilities() -> ServerCapabilities {
    ServerCapabilities {
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        document_symbol_provider: Some(OneOf::Left(true)),
        text_document_sync: Some(TextDocumentSyncCapability::Options(
            TextDocumentSyncOptions {
                open_close: Some(true),
                change: Some(TextDocumentSyncKind::INCREMENTAL),
                will_save: None,
                will_save_wait_until: None,
                save: None,
            },
        )),
        ..Default::default()
    }
}

static HANDLERS: &'static [RequestHandlerFn] = &[
    request::HoverRequest::handle_request,
    request::DocumentSymbolRequest::handle_request,
];

static NOTIFICATION_HANDLERS: &'static [NotificationHandlerFn] = &[
    notification::DidOpenTextDocument::handle_notification,
    notification::DidCloseTextDocument::handle_notification,
    notification::DidChangeTextDocument::handle_notification,
];

enum RequestHandlerResult {
    Success(RequestId, serde_json::Value),
    Failure(RequestId, ResponseError),
    NotInterested(Request),
}

enum NotificationHandlerResult {
    Success(Option<Notification>),
    Failure(ResponseError),
    NotInterested(Notification),
}

type RequestHandlerFn = fn(Request, &mut LspState) -> RequestHandlerResult;
type NotificationHandlerFn = fn(Notification, &mut LspState) -> NotificationHandlerResult;

pub fn run_lsp() -> AppResult<()> {
    // Note that  we must have our logging only write out to stderr.

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

    for msg in &connection.receiver {
        // eprintln!("got msg: {}", serde_json::to_string_pretty(&msg).unwrap());
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    return Ok(());
                }
                // eprintln!("got request: {req:?}");
                match handle_request(req, &mut state).into() {
                    Ok(response) => connection.sender.send(Message::Response(response))?,
                    Err(req) => eprintln!("unhandled request {req:?}"),
                }
            }
            Message::Response(resp) => {
                eprintln!("got response: {resp:?}");
            }
            Message::Notification(not) => {
                // eprintln!("got notification: {not:?}");
                match handle_notification(not, &mut state) {
                    Ok(None) => {}
                    Ok(Some(response)) => {
                        connection.sender.send(Message::Notification(response))?;
                    }
                    Err(not) => eprintln!("unhandled notification {not:?}"),
                }
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

fn handle_notification(
    mut notification: Notification,
    state: &mut LspState,
) -> Result<Option<Notification>, Notification> {
    for handler in NOTIFICATION_HANDLERS {
        use NotificationHandlerResult::*;
        match handler(notification, state) {
            NotInterested(it) => notification = it, // let the next handler try
            Success(reply) => {
                return Ok(reply);
            }
            Failure(err) => {
                return Ok(Some(lsp_server::Notification::new(
                    lsp_types::notification::LogMessage::METHOD.to_owned(),
                    err,
                )));
            }
        }
    }
    Err(notification)
}

trait RequestHandler: lsp_types::request::Request
where
    Self::Params: std::fmt::Debug,
{
    fn execute(params: Self::Params, state: &mut LspState) -> Result<Self::Result, ResponseError>;

    fn handle_request(req: Request, state: &mut LspState) -> RequestHandlerResult {
        use RequestHandlerResult::*;
        match req.extract(Self::METHOD) {
            Ok((id, params)) => match Self::execute(params, state) {
                Ok(result) => {
                    let result = serde_json::to_value(&result).unwrap();
                    Success(id, result)
                }
                Err(error) => Failure(id, error),
            },
            Err(err @ ExtractError::JsonError { .. }) => panic!("{err:?}"), // maybe we should skip malformed json gracefully?
            Err(ExtractError::MethodMismatch(req)) => NotInterested(req),
        }
    }
}

impl RequestHandler for lsp_types::request::HoverRequest {
    fn execute(
        _params: Self::Params,
        _state: &mut LspState,
    ) -> Result<Option<lsp_types::Hover>, ResponseError> {
        Ok(Some(Hover {
            contents: HoverContents::Scalar(MarkedString::String(
                "You are indeed hovering".to_owned(),
            )),
            range: None,
        }))
    }
}

impl RequestHandler for lsp_types::request::DocumentSymbolRequest {
    fn execute(
        params: Self::Params,
        _state: &mut LspState,
    ) -> Result<Option<lsp_types::DocumentSymbolResponse>, ResponseError> {
        #[allow(deprecated)]
        // `deprecated` is deprecated (ironic). But since we can't _not_ use it when constructing a value, we need to shut up the warnings here
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
        Ok(Some(DocumentSymbolResponse::Flat(vec![info])))
    }
}

trait NotificationHandler: lsp_types::notification::Notification
where
    Self::Params: std::fmt::Debug,
{
    fn execute(
        params: Self::Params,
        state: &mut LspState,
    ) -> Result<Option<Notification>, ResponseError>;

    fn handle_notification(
        notification: lsp_server::Notification,
        state: &mut LspState,
    ) -> NotificationHandlerResult {
        use NotificationHandlerResult::*;
        match notification.extract(Self::METHOD) {
            Ok(params) => match Self::execute(params, state) {
                Ok(reply) => Success(reply),
                Err(err) => Failure(err),
            },
            Err(err @ ExtractError::JsonError { .. }) => panic!("{err:?}"), // maybe we should skip malformed json gracefully?
            Err(ExtractError::MethodMismatch(req)) => NotInterested(req),
        }
    }
}

impl NotificationHandler for lsp_types::notification::DidOpenTextDocument {
    fn execute(
        params: Self::Params,
        state: &mut LspState,
    ) -> Result<Option<Notification>, ResponseError> {
        let text = params.text_document.text;
        let tree = state
            .parser
            .parse(&text, None)
            .expect("parsing should always work");
        let old = state
            .documents
            .insert(params.text_document.uri, Document { tree, text });
        assert!(old.is_none());
        Ok(None)
    }
}

impl NotificationHandler for lsp_types::notification::DidCloseTextDocument {
    fn execute(
        params: Self::Params,
        state: &mut LspState,
    ) -> Result<Option<Notification>, ResponseError> {
        let old = state.documents.remove(&params.text_document.uri);
        assert!(old.is_some());
        Ok(None)
    }
}

impl NotificationHandler for lsp_types::notification::DidChangeTextDocument {
    fn execute(
        params: Self::Params,
        state: &mut LspState,
    ) -> Result<Option<Notification>, ResponseError> {
        let doc = state
            .documents
            .get_mut(&params.text_document.uri)
            .expect("we should have put it in there with during didOpen");
        for change in params.content_changes {
            let edit = change
                .range
                .map(|range| edit_range(&doc, range, &change.text))
                .unwrap_or_else(|| edit_whole_document(&doc, &change.text));
            doc.tree.edit(&edit);
            doc.text
                .replace_range(edit.start_byte..edit.old_end_byte, &change.text);
        }
        doc.tree = state
            .parser
            .parse(&doc.text, Some(&doc.tree))
            .expect("parsing must work");
        Ok(None)
    }
}

fn point(pos: &Position) -> tree_sitter::Point {
    tree_sitter::Point {
        row: pos.line as usize,
        column: pos.character as usize,
    }
}

fn edit_range(doc: &Document, range: Range, new_text: &str) -> tree_sitter::InputEdit {
    let start_byte = doc
        .byte_for(range.start)
        .expect("range must be within document");
    let old_end_byte = doc
        .byte_for(range.end)
        .expect("range must be within document");
    let new_end_byte = start_byte + new_text.bytes().len();

    let start_position = point(&range.start);
    let old_end_position = point(&range.end);
    let mut new_end_position = start_position.clone();
    for char in new_text.chars() {
        if char == '\n' {
            new_end_position.row += 1;
            new_end_position.column = 0;
        } else {
            new_end_position.column += 1;
        }
    }

    tree_sitter::InputEdit {
        start_byte,
        old_end_byte,
        new_end_byte,
        start_position,
        old_end_position,
        new_end_position,
    }
}

fn edit_whole_document(doc: &Document, new_text: &str) -> InputEdit {
    InputEdit {
        start_byte: 0,
        old_end_byte: doc.text.len(),
        new_end_byte: new_text.len(),
        start_position: Point::new(0, 0),
        old_end_position: Point::new(0, 0),
        new_end_position: Point::new(0, 0),
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
