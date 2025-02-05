use crate::AppResult;
use line_index::WideEncoding;
use lsp_server::{
    Connection, ExtractError, Message, Notification, Request, RequestId, Response, ResponseError,
};
use lsp_types::*;
use state::State;
use std::{ops::Not, path::Path};

mod document;
mod file_watching;
mod links;
mod location;
mod notification_handlers;
mod request_handlers;
mod salsa;
mod scopes;
mod shared;
mod state;

// For that extra bit of convenience
pub(crate) type SharedState = shared::SharedValue<state::State>;

macro_rules! try_request_handlers {
    ($request:ident, $state:ident => $($handler:ident),+$(,)?) => {
        $(
        let $request = match $handler::handle_request($request, $state) {
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
    ($notification:ident, $state:ident => $($handler:ident),+$(,)?) => {
        $(
        let $notification = match $handler::handle_notification($notification, $state) {
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
        workspace_symbol_provider: Some(OneOf::Left(true)),
        definition_provider: Some(OneOf::Left(true)),
        references_provider: Some(OneOf::Left(true)),
        text_document_sync: Some(TextDocumentSyncCapability::Options(
            TextDocumentSyncOptions {
                open_close: Some(true),
                change: Some(TextDocumentSyncKind::INCREMENTAL),
                will_save: None,
                will_save_wait_until: None,
                save: None,
            },
        )),
        completion_provider: Some(CompletionOptions {
            resolve_provider: Some(false),
            trigger_characters: Some(vec!["->", "-> "].into_iter().map(str::to_string).collect()),
            all_commit_characters: None,
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: Some(false),
            },
            completion_item: None,
        }),
        position_encoding: find_utf8(params).or(Some(PositionEncodingKind::UTF16)),
        ..Default::default()
    }
}

/// Some clients may say they support file watching, but they don't (or do it badly)
/// For those we override the client capabilities and do the watching ourselves.
// TODO: move to config at some point
// IDEA: Perhaps just provide a config to force server side watching directly
fn force_server_file_watcher(client_info: &ClientInfo) -> bool {
    // https://github.com/helix-editor/helix/discussions/11903
    client_info.name == "helix"
}

// Add request and notification handlers here
fn handle_request(request: Request, state: &SharedState) -> Result<Response, Request> {
    use request::*;
    try_request_handlers! { request, state =>
        HoverRequest,
        DocumentSymbolRequest,
        WorkspaceSymbolRequest,
        Completion,
        GotoDefinition,
        References,
    }
}

fn handle_notification(
    notification: Notification,
    state: &SharedState,
) -> Result<(), Notification> {
    use notification::*;
    try_notification_handlers! { notification, state =>
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

    let qualified_names = init_params
        .capabilities
        .text_document
        .and_then(|it| it.document_symbol)
        .and_then(|it| it.hierarchical_document_symbol_support)
        .unwrap_or(false)
        .not();

    let state = shared::SharedValue::new(State::new(wide_encoding, qualified_names));

    if let Err(e) = client_connection.initialize_finish(init_id, init_result) {
        if e.channel_is_disconnected() {
            client_io_threads.join()?;
        }
        return Err(e.into());
    };

    let mut workspace_folders: Vec<_> = init_params
        .workspace_folders
        .unwrap_or_default()
        .iter()
        .map(|it| it.uri.path().as_str().to_owned())
        .collect();
    if workspace_folders.is_empty() {
        #[allow(deprecated)] // just a fallback
        if let Some(workpace_root) = init_params.root_uri {
            workspace_folders.push(workpace_root.path().as_str().to_owned());
        }
    }
    if workspace_folders.is_empty() {
        #[allow(deprecated)] // just a fallback
        if let Some(workpace_root) = init_params.root_path {
            workspace_folders.push(workpace_root);
        }
    }
    if workspace_folders.is_empty() {
        workspace_folders.push(".".to_owned());
    }

    let workspace_folders: Vec<_> = workspace_folders
        .into_iter()
        .map(|it| Path::new(&it).to_path_buf())
        .collect();

    eprintln!("Workspace Folders: {workspace_folders:?}");
    for path in workspace_folders.iter() {
        file_watching::read_initial_files(&path, &state)?;
    }

    let client_can_watch_files = init_params
        .capabilities
        .workspace
        .and_then(|it| it.did_change_watched_files)
        .and_then(|it| it.dynamic_registration)
        .unwrap_or(false);
    let force_server_side_watching = init_params
        .client_info
        .as_ref()
        .map(force_server_file_watcher)
        .unwrap_or(false);
    let file_watcher = if client_can_watch_files && !force_server_side_watching {
        eprintln!("relying on client for file watching");
        file_watching::register_file_change_notification(&client_connection, workspace_folders)
            .expect(
                "If this doesn't work, it means sending doesn't work at all. No need to go on.",
            );
        None
    } else {
        eprintln!("relying on server for file watching");
        Some(file_watching::start_file_watcher(
            state.clone(),
            workspace_folders,
        ))
    };

    // Ladies and gentlemen, the main loop:
    while let Ok(msg) = client_connection.receiver.recv() {
        if let Message::Request(ref req) = msg {
            if client_connection.handle_shutdown(req)? {
                continue;
            }
        }
        let handled = handle_message(msg, &state);
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
    drop(client_connection);
    client_io_threads.join()?;
    eprintln!("shutting down server state");

    Ok(())
}

pub(crate) fn handle_message(
    msg: lsp_server::Message,
    state: &SharedState,
) -> Option<lsp_server::Message> {
    match msg {
        Message::Request(req) => {
            // eprintln!("got request: {req:?}");
            match handle_request(req, state).into() {
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
            match handle_notification(not, state) {
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
    fn execute(params: Self::Params, state: &SharedState) -> Result<Self::Result, ResponseError>;

    fn handle_request(req: Request, state: &SharedState) -> RequestHandlerResult {
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
    fn execute(params: Self::Params, state: &SharedState) -> Result<(), ResponseError>;

    fn handle_notification(
        notification: lsp_server::Notification,
        state: &SharedState,
    ) -> NotificationHandlerResult {
        use NotificationHandlerResult::*;
        match notification.extract(Self::METHOD) {
            Ok(params) => match Self::execute(params, state) {
                Ok(()) => Success,
                Err(err) => Failure(err),
            },
            Err(err @ ExtractError::JsonError { .. }) => panic!("{err:?}"), // maybe we should skip malformed json gracefully?
            Err(ExtractError::MethodMismatch(req)) => NotInterested(req),
        }
    }
}
