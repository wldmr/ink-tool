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
    for walk_result in walkdir::WalkDir::new(root) {
        let path = match walk_result {
            Ok(ref it) => it.path(),
            Err(e) => {
                eprintln!("file read error: {e}");
                continue;
            }
        };

        if path.is_file() && path.extension().is_some_and(|ext| ext == "ink") {
            let path = std::path::absolute(path).expect("file should have a proper path");
            let path = path.to_str().expect("we should get proper file paths");
            let uri = Uri::from_str(&format!("file://{path}")).unwrap();
            match std::fs::read_to_string(path) {
                Ok(text) => {
                    let mut state = state.lock()?;
                    state.edit(uri, vec![(None, text)])?;
                }
                Err(err) => {
                    eprintln!("file read error: {err:?}");
                    continue;
                }
            }
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
        })
        .unwrap(),
    };
    eprintln!(
        "dynamic registration request: {}",
        serde_json::to_string_pretty(&request).unwrap()
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
            for path in inks {
                let path = std::path::absolute(path).expect("file should have a proper path");
                let path = path.to_str().expect("we should get proper file paths");
                let uri = Uri::from_str(&format!("file://{path}")).unwrap();
                match kind {
                    WatchEventKind::Edit => {
                        if let Err(err) = update_from_disk(path, uri, &state) {
                            eprintln!("document update error: {err:?}");
                            continue;
                        }
                    }
                    WatchEventKind::Forget => {
                        if let Err(err) = forget(uri, &state) {
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

fn update_from_disk(
    path: impl AsRef<std::path::Path>,
    uri: Uri,
    state: &SharedState,
) -> AppResult<()> {
    let text = std::fs::read_to_string(path)?;
    let mut state = state.lock()?;
    state.edit(uri, vec![(None, text)]).map_err(Into::into)
}

fn forget(uri: Uri, state: &SharedState) -> AppResult<()> {
    let mut state = state.lock()?;
    state.forget(uri).map_err(Into::into)
}
