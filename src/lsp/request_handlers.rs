use super::state::DocumentNotFound;
use super::RequestHandler;
use super::SharedState;
use lsp_server::ResponseError;
use lsp_types::*;

impl From<DocumentNotFound> for ResponseError {
    fn from(value: DocumentNotFound) -> Self {
        Self {
            code: lsp_server::ErrorCode::RequestFailed as i32,
            message: value.to_string(),
            data: serde_json::to_value(value.0.as_str()).ok(),
        }
    }
}

impl RequestHandler for request::HoverRequest {
    fn execute(
        _params: Self::Params,
        _state: &SharedState,
    ) -> Result<Option<Hover>, ResponseError> {
        Ok(Some(Hover {
            contents: HoverContents::Scalar(MarkedString::String(
                "You are indeed hovering".to_owned(),
            )),
            range: None,
        }))
    }
}

impl RequestHandler for request::DocumentSymbolRequest {
    fn execute(
        params: Self::Params,
        state: &SharedState,
    ) -> Result<Option<DocumentSymbolResponse>, ResponseError> {
        let response = state
            .lock()?
            .document_symbols(params.text_document.uri)?
            .and_then(|it| it.children)
            .map(DocumentSymbolResponse::Nested);
        Ok(response)
    }
}

impl RequestHandler for request::WorkspaceSymbolRequest {
    fn execute(
        params: Self::Params,
        state: &SharedState,
    ) -> Result<Option<WorkspaceSymbolResponse>, ResponseError> {
        let symbols = state.lock()?.workspace_symbols(params.query);
        Ok(Some(WorkspaceSymbolResponse::Nested(symbols)))
    }
}
