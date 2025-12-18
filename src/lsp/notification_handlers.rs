use super::{NotificationHandler, SharedState};
use lsp_server::ResponseError;

impl NotificationHandler for lsp_types::notification::DidOpenTextDocument {
    fn execute(params: Self::Params, state: &SharedState) -> Result<(), ResponseError> {
        let uri = params.text_document.uri;
        let mut state = state.lock()?;
        state.edit(uri, params.text_document.text);
        Ok(())
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
        let mut state = state.lock()?;
        state.edits(params.text_document.uri, params.content_changes);
        Ok(())
    }
}
