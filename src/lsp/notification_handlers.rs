use super::document::Document;
use super::state::ServerState;

use lsp_server::ResponseError;

use lsp_server::Notification;

use super::NotificationHandler;

impl NotificationHandler for lsp_types::notification::DidOpenTextDocument {
    fn execute(
        params: Self::Params,
        state: &mut ServerState,
    ) -> Result<Option<Notification>, ResponseError> {
        let old = state.documents.insert(
            params.text_document.uri,
            Document::new(params.text_document.text, &mut state.parser),
        );
        assert!(old.is_none());
        Ok(None)
    }
}

impl NotificationHandler for lsp_types::notification::DidCloseTextDocument {
    fn execute(
        params: Self::Params,
        state: &mut ServerState,
    ) -> Result<Option<Notification>, ResponseError> {
        let old = state.documents.remove(&params.text_document.uri);
        assert!(old.is_some());
        Ok(None)
    }
}

impl NotificationHandler for lsp_types::notification::DidChangeTextDocument {
    fn execute(
        params: Self::Params,
        state: &mut ServerState,
    ) -> Result<Option<Notification>, ResponseError> {
        let doc = state
            .documents
            .get_mut(&params.text_document.uri)
            .ok_or_else(|| ResponseError {
                code: lsp_server::ErrorCode::RequestFailed as i32,
                message: format!("Unopened document: {:?}", &params.text_document.uri),
                data: None,
            })?;
        let edits = params
            .content_changes
            .into_iter()
            .map(|it| (it.range, it.text))
            .collect();
        doc.apply_edits(edits, &mut state.parser);
        Ok(None)
    }
}
