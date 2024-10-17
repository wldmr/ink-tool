use lsp_server::ResponseError;
use lsp_types::*;

use super::state;
use super::RequestHandler;

impl RequestHandler for lsp_types::request::HoverRequest {
    fn execute(
        _params: Self::Params,
        _sender: &crossbeam::channel::Sender<state::Request>,
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
        _sender: &crossbeam::channel::Sender<state::Request>,
    ) -> Result<Option<lsp_types::DocumentSymbolResponse>, ResponseError> {
        #[allow(deprecated)]
        // `deprecated` is deprecated (ironic). But since we can't _not_ use it when constructing a value, we need to shut up the warnings here
        let info = SymbolInformation {
            name: "Boo".to_owned(),
            kind: SymbolKind::FIELD,
            tags: None,
            deprecated: None,
            location: Location {
                uri: params.text_document.uri,
                range: Range::new(Position::new(0, 0), Position::new(0, 0)),
            },
            container_name: None,
        };
        Ok(Some(DocumentSymbolResponse::Flat(vec![info])))
    }
}
