use super::response_error;
use super::RequestHandler;
use super::SharedState;
use lsp_server::ResponseError;
use lsp_types::*;

impl RequestHandler for lsp_types::request::HoverRequest {
    fn execute(
        _params: Self::Params,
        _state: &SharedState,
    ) -> Result<Option<lsp_types::Hover>, ResponseError> {
        Ok(Some(Hover {
            contents: HoverContents::Scalar(MarkedString::String(
                "You are indeed hovering".to_owned(),
            )),
            range: None,
        }))
    }
}

impl RequestHandler for lsp_types::request::DocumentSymbolRequest {
    fn execute(
        params: Self::Params,
        state: &SharedState,
    ) -> Result<Option<lsp_types::DocumentSymbolResponse>, ResponseError> {
        let maybe_toplevel = state
            .lock()?
            .document_symbols(&params.text_document.uri)
            .map_err(|msg| response_error(lsp_server::ErrorCode::RequestFailed, msg))?;
        Ok(maybe_toplevel
            .and_then(|it| it.children)
            .map(DocumentSymbolResponse::Nested))
    }
}

impl RequestHandler for lsp_types::request::WorkspaceSymbolRequest {
    fn execute(
        params: Self::Params,
        state: &SharedState,
    ) -> Result<Option<lsp_types::WorkspaceSymbolResponse>, ResponseError> {
        let symbols = state.lock()?.workspace_symbols(params.query);
        Ok(Some(WorkspaceSymbolResponse::Nested(symbols)))
    }
}
