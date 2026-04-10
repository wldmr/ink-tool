use crate::lsp::idset::Id;
use crate::lsp::salsa::{self, InkGetters as _};
use crate::lsp::shared::SharedValue;
use crate::lsp::state::State;
use crate::AppResult;
use lsp_server::{Message, Notification};
use lsp_types::{PublishDiagnosticsParams, Uri};
use mini_milc::Revision;
use std::collections::HashMap;
use std::ops::ControlFlow;

/* TODO: Proper debounce
> In a language server written in Rust, what would be a good way to “debounce”
> updates after an edit. Let’s say I have a thread that computes diagnostics for
> a project. How do I continually tell that thread “not yet” while the user is
> typing, and then fire the update after, say 150ms of inactivity?

The standard pattern is to use a channel where sending a message resets a
deadline, and the worker thread waits on that deadline:

``` rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

enum DiagnosticMessage {
    Recompute,
    Shutdown,
}

fn spawn_diagnostic_thread() -> mpsc::Sender<DiagnosticMessage> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        loop {
            // Block until we get the first message
            match rx.recv().unwrap() {
                DiagnosticMessage::Shutdown => break,
                DiagnosticMessage::Recompute => {}
            }

            // Drain additional messages until 150ms of silence
            loop {
                match rx.recv_timeout(Duration::from_millis(150)) {
                    Ok(DiagnosticMessage::Shutdown) => return,
                    Ok(DiagnosticMessage::Recompute) => continue, // reset the timer
                    Err(mpsc::RecvTimeoutError::Timeout) => break, // 150ms passed, fire!
                    Err(mpsc::RecvTimeoutError::Disconnected) => return,
                }
            }

            compute_diagnostics();
        }
    });

    tx
}
```

The key insight is the inner loop: every time a new `Recompute` arrives within
the 150ms window, `recv_timeout` returns `Ok` and you `continue`, which restarts
the 150ms wait. Only when the timeout elapses with no new messages do you break
out and actually run the computation.

On the caller side, every keystroke just sends:

``` rust
tx.send(DiagnosticMessage::Recompute).ok();
```

**A few notes:**

- If you’re in an async context (tokio), swap in `tokio::sync::mpsc` and
  `tokio::time::timeout` — same logic, but `async/await`.
- `crossbeam-channel` also works well here and has a cleaner `select!` macro if
  you need to multiplex multiple signal types.
- For a real LSP you might want to carry a `DocumentVersion` in the message and
  skip computation if the version that triggered the timeout is already stale.
*/

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
                let latest_diagnostics = state.db.file_diagnostics(docid);

                if let Some(rev) = state.db.changed_at(salsa::file_diagnostics { docid }) {
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
