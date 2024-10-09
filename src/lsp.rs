use lsp_types::*;

use lsp_server::{
    Connection, ExtractError, Message, Notification, Request, RequestId, Response, ResponseError,
};

use crate::AppResult;

mod document;
mod notification_handlers;
mod request_handlers;
mod state;
mod tree;

// *** Config Area: Define Server behaviors here ***

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
        position_encoding: Some(PositionEncodingKind::UTF16),
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

// *** End Config Area ***

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

type RequestHandlerFn = fn(Request, &mut state::ServerState) -> RequestHandlerResult;
type NotificationHandlerFn = fn(Notification, &mut state::ServerState) -> NotificationHandlerResult;

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
    let params: InitializeParams = serde_json::from_value(initialization_params).unwrap();
    eprintln!(
        "initparams: {}",
        serde_json::to_string_pretty(&params).unwrap()
    );
    let mut state = state::ServerState::new();

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

fn handle_request(
    mut request: Request,
    state: &mut state::ServerState,
) -> Result<Response, Request> {
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
    state: &mut state::ServerState,
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
                    <lsp_types::notification::LogMessage as notification::Notification>::METHOD
                        .to_owned(),
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
    fn execute(
        params: Self::Params,
        state: &mut state::ServerState,
    ) -> Result<Self::Result, ResponseError>;

    fn handle_request(req: Request, state: &mut state::ServerState) -> RequestHandlerResult {
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

trait NotificationHandler: lsp_types::notification::Notification
where
    Self::Params: std::fmt::Debug,
{
    fn execute(
        params: Self::Params,
        state: &mut state::ServerState,
    ) -> Result<Option<Notification>, ResponseError>;

    fn handle_notification(
        notification: lsp_server::Notification,
        state: &mut state::ServerState,
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
