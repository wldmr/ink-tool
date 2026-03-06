use crate::lsp::idset::Id;
use crate::lsp::salsa::{self, InkGetters as _};
use crate::lsp::shared::SharedValue;
use crate::lsp::state::State;
use crate::AppResult;
use lsp_server::{Message, Notification};
use lsp_types::{PublishDiagnosticsParams, Uri};
use mini_milc::{Db as _, Revision};
use std::collections::HashMap;
use std::ops::ControlFlow;

/// Starts checking the diagnostics for all Iink files in `state`
///
/// The `send` closure takes a message and signals if sending failed. The
/// `wait_for_shutdown` closure blocks however long it wants; when if returns
/// [`ControlFlow::Continue`], any changed files will be checked. On
/// [`ControlFlow::Break`], the function exits.
///
/// We use closures for all these things to abstract over all the different ways
/// that messages can be sent (std::sync, tokio, crossbeam, …).
pub fn start(
    state: SharedValue<State>,
    send: impl Fn(Message) -> AppResult<()>,
    wait_for_shutdown: impl Fn() -> ControlFlow<()>,
) {
    // keep track of when the diagnostics last changed, per file
    let mut latest = HashMap::<Id<Uri>, Revision>::new();
    loop {
        if wait_for_shutdown().is_break() {
            log::debug!("Diagnostics thread received shutdown signal.");
            break;
        }

        if let Ok(state) = state.lock() {
            let docs = state.db.doc_ids();

            for (docid, uri) in docs.pairs() {
                let errors_query = salsa::parse_errors { docid };
                let latest_diagnostics = state.db.get(errors_query);

                if let Some(rev) = state.db.changed_at(errors_query) {
                    let old = latest.insert(docid, rev);
                    if old.is_none_or(|it| it != rev) {
                        static METHOD: &'static str = <lsp_types::notification::PublishDiagnostics as lsp_types::notification::Notification>::METHOD;
                        let params = PublishDiagnosticsParams {
                            uri: uri.clone(),
                            diagnostics: latest_diagnostics.clone(),
                            version: None,
                        };
                        let params = match serde_json::to_value(params) {
                            Ok(ok) => ok,
                            Err(err) => {
                                log::error!("Couldn't convert diagnostics to JSON: {err:?}");
                                continue;
                            }
                        };
                        let notification = Message::Notification(Notification {
                            method: METHOD.to_string(),
                            params,
                        });
                        if let Err(err) = send(notification) {
                            log::error!("Notification error: {err:?}");
                        } else {
                            log::trace!("Sent updated parse errors for {}", uri.path().as_str());
                        }
                    }
                }
            }
        } else {
            log::error!("Couldn't aquire state, aborting diagnostics");
            break;
        }
    }
}
