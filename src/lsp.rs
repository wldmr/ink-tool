use crate::AppResult;
use line_index::WideEncoding;
use lsp_server::{
    Connection, ExtractError, Message, Notification, Request, RequestId, Response, ResponseError,
};
use lsp_types::*;
use request::Request as _;
use state::ServerState;

mod document;
mod notification_handlers;
mod request_handlers;
mod state;
mod tree;

// *** Config Area: Define Server behaviors here ***

const INK_GLOB: &str = "**/*.ink";

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

type RequestHandlerFn = fn(Request, &mut ServerState) -> RequestHandlerResult;
type NotificationHandlerFn = fn(Notification, &mut ServerState) -> NotificationHandlerResult;

pub fn run_lsp() -> AppResult<()> {
    // Note that  we must have our logging only write out to stderr.

    // Create the transport. Includes the stdio (stdin and stdout) versions but this could
    // also be implemented to use sockets or HTTP.
    let (connection, io_threads) = Connection::stdio();

    // Init

    let (init_id, params) = match connection.initialize_start() {
        Ok(it) => it,
        Err(e) => {
            if e.channel_is_disconnected() {
                io_threads.join()?;
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

    if let Err(e) = connection.initialize_finish(init_id, init_result) {
        if e.channel_is_disconnected() {
            io_threads.join()?;
        }
        return Err(e.into());
    };

    let can_watch_files = init_params
        .capabilities
        .workspace
        .and_then(|it| it.did_change_watched_files)
        .and_then(|it| it.dynamic_registration)
        .unwrap_or(false);

    if can_watch_files {
        let ink_files = lsp_types::FileSystemWatcher {
            glob_pattern: GlobPattern::String(INK_GLOB.into()),
            kind: None,
        };
        let options = lsp_types::DidChangeWatchedFilesRegistrationOptions {
            watchers: vec![ink_files],
        };
        let registration = Registration {
            id: "ink-files-watcher".into(),
            method: "workspace/didChangeWatchedFiles".into(),
            register_options: Some(serde_json::to_value(&options).unwrap()),
        };
        let dyn_reg_id: RequestId = 0.into();
        let request = Request {
            id: dyn_reg_id.clone(),
            method: request::RegisterCapability::METHOD.into(),
            params: serde_json::to_value(RegistrationParams {
                registrations: vec![registration],
            })
            .unwrap(),
        };
        eprintln!(
            "dynamic registration request: {}",
            serde_json::to_string_pretty(&request).unwrap()
        );
        let msg = Message::Request(request);
        connection.sender.send(msg).expect(
            "If this doesn't work, it means sending doesn't work at all. No need to go on.",
        );
    }

    let mut state = ServerState::new(wide_encoding);

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

fn handle_request(mut request: Request, state: &mut ServerState) -> Result<Response, Request> {
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
    state: &mut ServerState,
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
        state: &mut ServerState,
    ) -> Result<Self::Result, ResponseError>;

    fn handle_request(req: Request, state: &mut ServerState) -> RequestHandlerResult {
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
        state: &mut ServerState,
    ) -> Result<Option<Notification>, ResponseError>;

    fn handle_notification(
        notification: lsp_server::Notification,
        state: &mut ServerState,
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
