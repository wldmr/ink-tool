use super::{
    state::{self},
    NotificationHandler,
};
use lsp_server::ResponseError;

impl NotificationHandler for lsp_types::notification::DidOpenTextDocument {
    fn execute(
        params: Self::Params,
        sender: &crossbeam::channel::Sender<state::Request>,
    ) -> Result<(), ResponseError> {
        let edit = vec![(None, params.text_document.text)];
        sender
            .send((
                state::Command::EditDocument(params.text_document.uri, edit),
                None,
            ))
            .map_err(|e| error(lsp_server::ErrorCode::InternalError, e.to_string()))
    }
}

impl NotificationHandler for lsp_types::notification::DidCloseTextDocument {
    fn execute(
        _params: Self::Params,
        _sender: &crossbeam::channel::Sender<state::Request>,
    ) -> Result<(), ResponseError> {
        // not sure if we need to do anything
        Ok(())
    }
}

impl NotificationHandler for lsp_types::notification::DidChangeTextDocument {
    fn execute(
        params: Self::Params,
        sender: &crossbeam::channel::Sender<state::Request>,
    ) -> Result<(), ResponseError> {
        let edits = params
            .content_changes
            .into_iter()
            .map(|it| (it.range, it.text))
            .collect();
        sender
            .send((
                state::Command::EditDocument(params.text_document.uri, edits),
                None,
            ))
            .map_err(|e| error(lsp_server::ErrorCode::InternalError, e.to_string()))
    }
}

fn error(code: lsp_server::ErrorCode, message: impl Into<String>) -> ResponseError {
    ResponseError {
        code: code as i32,
        message: message.into(),
        data: None,
    }
}
