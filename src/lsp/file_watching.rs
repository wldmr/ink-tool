use super::SharedState;
use crate::{
    lsp::{DID_CHANGE_WATCHED_FILES, INK_GLOB},
    AppResult,
};
use lsp_server::{Connection, Message, Request};
use lsp_types::{
    request::{self, Request as _},
    GlobPattern, Registration, RegistrationParams, Uri,
};
use std::str::FromStr;

pub(crate) fn read_initial_files(root: &std::path::Path, state: &SharedState) -> AppResult<()> {
    // We'll liberally `?` out of any error. Failing to read initial would leave the server in a weird state.
    for dir_entry in walkdir::WalkDir::new(root) {
        let dir_entry = dir_entry?;
        let path = dir_entry.path();
        let mut state = state.lock()?;
        if path.is_file() && path.extension().is_some_and(|ext| ext == "ink") {
            let path = std::path::absolute(path)?;
            let path = path.to_str().ok_or("path wasn't a proper UTF-8 string")?;
            let uri = Uri::from_str(&format!("file://{path}"))?;
            let text = std::fs::read_to_string(path)?;
            state.edit(uri, vec![(None, text)])?;
        }
    }
    Ok(())
}

pub(crate) fn register_file_change_notification(client_connection: &Connection) -> AppResult<()> {
    let ink_files = lsp_types::FileSystemWatcher {
        glob_pattern: GlobPattern::String(INK_GLOB.into()),
        kind: None,
    };
    let watch_files = Registration {
        id: "ink-files-watcher".into(),
        method: DID_CHANGE_WATCHED_FILES.into(),
        register_options: Some(
            serde_json::to_value(lsp_types::DidChangeWatchedFilesRegistrationOptions {
                watchers: vec![ink_files],
            })
            .unwrap(),
        ),
    };
    let request = Request {
        id: 0.into(),
        method: request::RegisterCapability::METHOD.into(),
        params: serde_json::to_value(RegistrationParams {
            registrations: vec![watch_files],
        })?,
    };
    eprintln!(
        "dynamic registration request: {}",
        serde_json::to_string_pretty(&request)?
    );
    client_connection.sender.send(Message::Request(request))?;
    Ok(())
}

pub(crate) fn start_file_watcher(
    root: &std::path::Path,
    state: super::SharedState,
) -> AppResult<impl notify::Watcher> {
    use notify::Watcher as _;
    use std::str::FromStr;

    #[derive(Debug)]
    enum WatchEventKind {
        Edit,
        Forget,
    }

    let mut watcher = notify::recommended_watcher(move |res| match res {
        Ok(notify::Event { kind, paths, .. }) => {
            let kind = match kind {
                notify::EventKind::Modify(notify::event::ModifyKind::Data(_)) => {
                    WatchEventKind::Edit
                }
                notify::EventKind::Remove(notify::event::RemoveKind::File) => {
                    WatchEventKind::Forget
                }
                _ => return,
            };
            let inks = paths
                .iter()
                .filter(|it| it.extension().is_some_and(|ext| ext == "ink"));
            let mut state = state
                .lock()
                .expect("we should be able to get a lock on the state");
            for path in inks {
                let path = std::path::absolute(path).expect("file should have a proper path");
                let path = path.to_str().expect("we should get proper file paths");
                let uri = Uri::from_str(&format!("file://{path}")).unwrap();
                match kind {
                    WatchEventKind::Edit => {
                        let result = std::fs::read_to_string(path)
                            .map_err(|e| e.to_string())
                            .and_then(|text| state.edit(uri, vec![(None, text)]));
                        if let Err(err) = result {
                            eprintln!("document update error: {err:?}");
                            continue;
                        }
                    }
                    WatchEventKind::Forget => {
                        if let Err(err) = state.forget(uri) {
                            eprintln!("document remove error: {err:?}");
                            continue;
                        }
                    }
                };
            }
        }
        Err(e) => eprintln!("watch error: {:?}", e),
    })?;
    watcher.watch(root, notify::RecursiveMode::Recursive)?;
    Ok(watcher)
}
