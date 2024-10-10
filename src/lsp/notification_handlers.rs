use super::{state::ServerState, NotificationHandler};
use lsp_server::{Notification, ResponseError};

impl NotificationHandler for lsp_types::notification::DidOpenTextDocument {
    fn execute(
        params: Self::Params,
        state: &mut ServerState,
    ) -> Result<Option<Notification>, ResponseError> {
        state
            .open_document(params.text_document.uri, params.text_document.text)
            .map(|()| None)
            .map_err(request_failed)
    }
}

impl NotificationHandler for lsp_types::notification::DidCloseTextDocument {
    fn execute(
        params: Self::Params,
        state: &mut ServerState,
    ) -> Result<Option<Notification>, ResponseError> {
        state
            .close_document(params.text_document.uri)
            .map(|_| None)
            .map_err(request_failed)
    }
}

impl NotificationHandler for lsp_types::notification::DidChangeTextDocument {
    fn execute(
        params: Self::Params,
        state: &mut ServerState,
    ) -> Result<Option<Notification>, ResponseError> {
        let edits = params
            .content_changes
            .into_iter()
            .map(|it| (it.range, it.text))
            .collect();
        state
            .edit_document(params.text_document.uri, edits)
            .map(|()| None)
            .map_err(request_failed)
    }
}

fn request_failed(message: String) -> ResponseError {
    ResponseError {
        code: lsp_server::ErrorCode::RequestFailed as i32,
        message,
        data: None,
    }
}
