use super::{response_error, NotificationHandler, SharedState};
use lsp_server::ResponseError;

impl NotificationHandler for lsp_types::notification::DidOpenTextDocument {
    fn execute(params: Self::Params, state: &SharedState) -> Result<(), ResponseError> {
        let uri = params.text_document.uri;
        let edit = vec![(None, params.text_document.text)];
        let mut state = state.lock()?;
        state
            .edit(uri, edit)
            .map_err(|e| response_error(lsp_server::ErrorCode::InternalError, e.to_string()))
    }
}

impl NotificationHandler for lsp_types::notification::DidCloseTextDocument {
    fn execute(_params: Self::Params, _state: &SharedState) -> Result<(), ResponseError> {
        // not sure if we need to do anything
        Ok(())
    }
}

impl NotificationHandler for lsp_types::notification::DidChangeTextDocument {
    fn execute(params: Self::Params, state: &SharedState) -> Result<(), ResponseError> {
        let edits = params
            .content_changes
            .into_iter()
            .map(|it| (it.range, it.text))
            .collect();
        let mut state = state.lock()?;
        state
            .edit(params.text_document.uri, edits)
            .map_err(|e| response_error(lsp_server::ErrorCode::InternalError, e.to_string()))
    }
}
