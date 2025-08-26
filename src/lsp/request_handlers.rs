use super::state::DocumentNotFound;
use super::state::GotoDefinitionError;
use super::RequestHandler;
use super::SharedState;
use lsp_server::ResponseError;
use lsp_types::*;
use std::error::Error;

impl From<DocumentNotFound> for ResponseError {
    fn from(value: DocumentNotFound) -> Self {
        Self {
            code: lsp_server::ErrorCode::RequestFailed as i32,
            message: value.to_string(),
            data: serde_json::to_value(value.0.as_str()).ok(),
        }
    }
}

impl From<GotoDefinitionError> for ResponseError {
    fn from(value: GotoDefinitionError) -> Self {
        Self {
            code: lsp_server::ErrorCode::RequestFailed as i32,
            message: value.to_string(),
            data: value
                .source()
                .map(ToString::to_string)
                .map(serde_json::to_value)
                .and_then(Result::ok),
        }
    }
}

type Response<T> = Result<T, ResponseError>;

// impl RequestHandler for request::HoverRequest {
//     fn execute(_params: Self::Params, _state: &SharedState) -> Response<Self::Result> {
//         Ok(Some(Hover {
//             contents: HoverContents::Scalar(MarkedString::String(
//                 "You are indeed hovering".to_owned(),
//             )),
//             range: None,
//         }))
//     }
// }

impl RequestHandler for request::DocumentSymbolRequest {
    fn execute(params: Self::Params, state: &SharedState) -> Response<Self::Result> {
        let response = state
            .lock()?
            .document_symbols(params.text_document.uri)?
            .and_then(|it| it.children)
            .map(DocumentSymbolResponse::Nested);
        Ok(response)
    }
}

impl RequestHandler for request::WorkspaceSymbolRequest {
    fn execute(params: Self::Params, state: &SharedState) -> Response<Self::Result> {
        let symbols = state.lock()?.workspace_symbols(params.query);
        Ok(Some(WorkspaceSymbolResponse::Nested(symbols)))
    }
}

// impl RequestHandler for request::Completion {
//     fn execute(params: Self::Params, state: &SharedState) -> Response<Self::Result> {
//         let completions = state.lock()?.completions(
//             params.text_document_position.text_document.uri,
//             params.text_document_position.position,
//         )?;
//         Ok(completions.map(CompletionResponse::Array))
//     }
// }

// impl RequestHandler for request::GotoDefinition {
//     fn execute(params: Self::Params, state: &SharedState) -> Response<Self::Result> {
//         use lsp_types::GotoDefinitionResponse::*;
//         let defs = state.lock()?.goto_definition(
//             &params.text_document_position_params.text_document.uri,
//             params.text_document_position_params.position,
//         )?;
//         let response = match defs.len() {
//             0 => None,
//             1 => Some(Scalar(defs.into_iter().next().unwrap())),
//             _ => Some(Array(defs)),
//         };
//         Ok(response)
//     }
// }

// impl RequestHandler for request::References {
//     fn execute(params: Self::Params, state: &SharedState) -> Response<Self::Result> {
//         let refs = state.lock()?.goto_references(
//             &params.text_document_position.text_document.uri,
//             params.text_document_position.position,
//         )?;
//         let response = match refs.len() {
//             0 => None,
//             _ => Some(refs),
//         };
//         Ok(response)
//     }
// }
