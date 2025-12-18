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

impl NotificationHandler for lsp_types::notification::DidChangeWatchedFiles {
    fn execute(params: Self::Params, state: &SharedState) -> Result<(), ResponseError> {
        use lsp_types::FileChangeType;

        let mut state = state.lock()?;
        let mut errors = Vec::new();

        for change in params.changes {
            match state.is_open(&change.uri) {
                Ok(true) => continue,
                Err(err) => errors.push(err.to_string()), // unknown file
                Ok(false) => { /* File not open in editor. See below. */ }
            }

            let path = change.uri.path().as_str();
            match change.typ {
                reason @ (FileChangeType::CREATED | FileChangeType::CHANGED) => {
                    log::debug!("Updating watched file {path}. Reason: {reason:?}");
                    let text = std::fs::read_to_string(path);
                    match text {
                        Ok(text) => state.edit(change.uri, text),
                        Err(err) => errors.push(err.to_string()),
                    }
                }
                FileChangeType::DELETED => {
                    log::debug!("Forgetting watched file {path}");
                    if let Err(err) = state.forget(change.uri) {
                        errors.push(format!("Was told to close unknown file: {err:?}"))
                    }
                }
                other => {
                    log::warn!("Unhandled file event {other:?}")
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(ResponseError {
                code: lsp_server::ErrorCode::RequestFailed as i32,
                message: "Error updating watched files".to_string(),
                data: Some(errors.into()),
            })
        }
    }
}
