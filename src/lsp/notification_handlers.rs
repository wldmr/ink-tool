use super::{NotificationHandler, SharedState};
use lsp_server::ResponseError;

impl NotificationHandler for lsp_types::notification::DidOpenTextDocument {
    fn execute(params: Self::Params, state: &SharedState) -> Result<(), ResponseError> {
        let uri = params.text_document.uri;
        log::debug!("Opening file {}", uri.path().as_str());
        let mut state = state.lock()?;
        // We treat this as idempotent. If we're told to open it, it'll be open afterwards.
        state.open(uri.clone());
        state.edit(uri, params.text_document.text);
        Ok(())
    }
}

impl NotificationHandler for lsp_types::notification::DidCloseTextDocument {
    fn execute(params: Self::Params, state: &SharedState) -> Result<(), ResponseError> {
        let uri = params.text_document.uri;
        log::debug!("Closing file {}", uri.path().as_str());
        // Reminder: This is idempotent.
        state.lock()?.close(uri);
        Ok(())
    }
}

impl NotificationHandler for lsp_types::notification::DidChangeTextDocument {
    fn execute(params: Self::Params, state: &SharedState) -> Result<(), ResponseError> {
        log::trace!(
            "Editing file {}(v{}): {:?}",
            params.text_document.uri.path().as_str(),
            params.text_document.version,
            params.content_changes,
        );
        let mut state = state.lock()?;
        state.edits(params.text_document.uri, params.content_changes);
        Ok(())
    }
}
