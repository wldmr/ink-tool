use crate::AppResult;
use line_index::WideEncoding;
use lsp_server::{
    Connection, ExtractError, Message, Notification, Request, RequestId, Response, ResponseError,
};
use lsp_types::*;
use state::State;

mod document;
mod file_watching;
mod notification_handlers;
mod request_handlers;
mod state;
mod tree;

macro_rules! try_request_handlers {
    ($request:ident, $sender:ident => $($handler:ident),+$(,)?) => {
        $(
        let $request = match $handler::handle_request($request, $sender) {
            RequestHandlerResult::NotInterested(it) => it, // let the next handler try
            RequestHandlerResult::Success(id, succ) => {
                return Ok(Response {
                    id,
                    result: Some(succ),
                    error: None,
                })
            }
            RequestHandlerResult::Failure(id, fail) => {
                return Ok(Response {
                    id,
                    result: None,
                    error: Some(fail),
                })
            }
        };
        )+
        Err($request)
    };
}

macro_rules! try_notification_handlers {
    ($notification:ident, $sender:ident => $($handler:ident),+$(,)?) => {
        $(
        let $notification = match $handler::handle_notification($notification, $sender) {
            NotificationHandlerResult::NotInterested(it) => it, // let the next handler try
            NotificationHandlerResult::Success => {
                return Ok(());
            }
            NotificationHandlerResult::Failure(err) => {
                eprintln!("{err:?}");
                return Ok(());
            }
        };
        )+
        Err($notification)
    };
}

// *** Config Area: Define Server behaviors here ***

const INK_GLOB: &str = "**/*.ink";
const DID_CHANGE_WATCHED_FILES: &str = "workspace/didChangeWatchedFiles";

fn server_capabilities(params: &InitializeParams) -> ServerCapabilities {
    /// This function only exists so we can use the ? operator.
    fn find_utf8(params: &InitializeParams) -> Option<PositionEncodingKind> {
        params
            .capabilities
            .general
            .as_ref()?
            .position_encodings
            .as_ref()?
            .iter()
            .find(|&it| it == &PositionEncodingKind::UTF8)
            .cloned()
    }

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
        position_encoding: find_utf8(params).or(Some(PositionEncodingKind::UTF16)),
        ..Default::default()
    }
}

/// Some clients may say they support file watching, but they don't (or do it badly)
/// For those we override the client capabilities and do the watching ourselves.
// TODO: move to config at some point
fn force_server_file_watcher(client_info: &ClientInfo) -> bool {
    // https://github.com/helix-editor/helix/discussions/11903
    client_info.name == "helix"
}

// Add request and notification handlers here
fn handle_request(
    request: Request,
    sender: &crossbeam::channel::Sender<state::Request>,
) -> Result<Response, Request> {
    use request::*;
    try_request_handlers! { request, sender =>
        HoverRequest,
        DocumentSymbolRequest,
    }
}

fn handle_notification(
    notification: Notification,
    sender: &crossbeam::channel::Sender<state::Request>,
) -> Result<(), Notification> {
    use notification::*;
    try_notification_handlers! { notification, sender =>
        DidOpenTextDocument,
        DidCloseTextDocument,
        DidChangeTextDocument,
    }
}

// *** End Config Area ***

enum RequestHandlerResult {
    Success(RequestId, serde_json::Value),
    Failure(RequestId, ResponseError),
    NotInterested(Request),
}

enum NotificationHandlerResult {
    Success,
    Failure(ResponseError),
    NotInterested(Notification),
}

pub fn run_lsp() -> AppResult<()> {
    // Note that  we must have our logging only write out to stderr.

    // Create the transport. Includes the stdio (stdin and stdout) versions but this could
    // also be implemented to use sockets or HTTP.
    let (client_connection, client_io_threads) = Connection::stdio();

    // Init
    let (init_id, params) = match client_connection.initialize_start() {
        Ok(it) => it,
        Err(e) => {
            if e.channel_is_disconnected() {
                client_io_threads.join()?;
            }
            return Err(e.into());
        }
    };
    let init_params: InitializeParams = serde_json::from_value(params).unwrap();
    let server_capabilities = server_capabilities(&init_params);

    let wide_encoding = match server_capabilities.position_encoding {
        Some(ref enc) if *enc == PositionEncodingKind::UTF8 => None,
        Some(ref enc) if *enc == PositionEncodingKind::UTF16 => Some(WideEncoding::Utf16),
        Some(ref enc) if *enc == PositionEncodingKind::UTF32 => Some(WideEncoding::Utf32),
        Some(ref other) => panic!("Unknown encoding '{other:?}'"),
        None => panic!("We must guarantee a position encoding!"),
    };

    let init_result = InitializeResult {
        capabilities: server_capabilities,
        server_info: Some(ServerInfo {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: Some(env!("CARGO_PKG_VERSION").to_string()),
        }),
    };
    let init_result = serde_json::to_value(&init_result)?;

    eprintln!(
        "init params: {}",
        serde_json::to_string_pretty(&init_params).unwrap()
    );
    eprintln!(
        "init result: {}",
        serde_json::to_string_pretty(&init_result).unwrap()
    );

    let server_command_channel = crossbeam::channel::unbounded();
    let server_state_handle = state::run(State::new(wide_encoding), server_command_channel.1)?;

    if let Err(e) = client_connection.initialize_finish(init_id, init_result) {
        if e.channel_is_disconnected() {
            client_io_threads.join()?;
        }
        return Err(e.into());
    };

    let workspace_root = std::path::Path::new(".");

    file_watching::read_initial_files(workspace_root, &server_command_channel.0);

    let client_can_watch_files = init_params
        .capabilities
        .workspace
        .and_then(|it| it.did_change_watched_files)
        .and_then(|it| it.dynamic_registration)
        .unwrap_or(false);
    let force = init_params
        .client_info
        .as_ref()
        .map(force_server_file_watcher)
        .unwrap_or(false);
    let file_watcher = if client_can_watch_files && !force {
        file_watching::register_file_change_notification(&client_connection).expect(
            "If this doesn't work, it means sending doesn't work at all. No need to go on.",
        );
        None
    } else {
        Some(file_watching::start_file_watcher(
            workspace_root,
            server_command_channel.0.clone(),
        ))
    };

    // Ladies and gentlemen, the main loop:
    while let Ok(msg) = client_connection.receiver.recv() {
        if let Message::Request(ref req) = msg {
            if client_connection.handle_shutdown(req)? {
                continue;
            }
        }
        let handled = handle_message(msg, &server_command_channel.0);
        if let Some(reply) = handled {
            let _ = client_connection.sender.send(reply);
        }
    }

    // Shut down gracefully.
    if file_watcher.is_some() {
        eprintln!("shutting down file watcher");
        drop(file_watcher);
    }
    eprintln!("shutting down client connection");
    client_io_threads.join()?;
    eprintln!("shutting down server state");
    server_state_handle.join().expect("come on man");

    Ok(())
}

pub(crate) fn handle_message(
    msg: lsp_server::Message,
    sender: &crossbeam::channel::Sender<state::Request>,
) -> Option<lsp_server::Message> {
    match msg {
        Message::Request(req) => {
            // eprintln!("got request: {req:?}");
            match handle_request(req, sender).into() {
                Ok(response) => Some(Message::Response(response)),
                Err(req) => {
                    eprintln!("unhandled request {req:?}");
                    None
                }
            }
        }
        Message::Response(resp) => {
            eprintln!("got response: {resp:?}");
            None
        }
        Message::Notification(not) => {
            // eprintln!("got notification: {not:?}");
            match handle_notification(not, sender) {
                Ok(()) => None,
                Err(not) => {
                    eprintln!("unhandled notification {not:?}");
                    None
                }
            }
        }
    }
}

trait RequestHandler: lsp_types::request::Request
where
    Self::Params: std::fmt::Debug,
{
    fn execute(
        params: Self::Params,
        sender: &crossbeam::channel::Sender<state::Request>,
    ) -> Result<Self::Result, ResponseError>;

    fn handle_request(
        req: Request,
        sender: &crossbeam::channel::Sender<state::Request>,
    ) -> RequestHandlerResult {
        use RequestHandlerResult::*;
        match req.extract(Self::METHOD) {
            Ok((id, params)) => match Self::execute(params, sender) {
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
        sender: &crossbeam::channel::Sender<state::Request>,
    ) -> Result<(), ResponseError>;

    fn handle_notification(
        notification: lsp_server::Notification,
        sender: &crossbeam::channel::Sender<state::Request>,
    ) -> NotificationHandlerResult {
        use NotificationHandlerResult::*;
        match notification.extract(Self::METHOD) {
            Ok(params) => match Self::execute(params, sender) {
                Ok(()) => Success,
                Err(err) => Failure(err),
            },
            Err(err @ ExtractError::JsonError { .. }) => panic!("{err:?}"), // maybe we should skip malformed json gracefully?
            Err(ExtractError::MethodMismatch(req)) => NotInterested(req),
        }
    }
}
