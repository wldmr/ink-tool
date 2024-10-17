use super::document::{DocumentEdit, InkDocument};
use crate::AppResult;
use crossbeam::channel::Receiver;
use line_index::WideEncoding;
use lsp_types::Uri;
use std::collections::HashMap;

pub type Request = (Command, Option<crossbeam::channel::Sender<Response>>);

/// Make something happen to the state
#[derive(Debug)]
pub(crate) enum Command {
    /// Change the content of this document (including on initial load)
    EditDocument(Uri, Vec<DocumentEdit>),
    /// Forget everything about this document
    ForgetDocument(Uri),
}

pub(crate) enum Response {
    Ok,
    Error(String),
}

pub(crate) struct State {
    wide_encoding: Option<WideEncoding>,
    documents: HashMap<Uri, InkDocument>,
}

impl State {
    pub fn new(wide_encoding: Option<WideEncoding>) -> Self {
        Self {
            documents: HashMap::new(),
            wide_encoding,
        }
    }

    fn handle(&mut self, cmd: Command) -> Response {
        match cmd {
            Command::EditDocument(uri, edits) => {
                let entry = self
                    .documents
                    .entry(uri)
                    .or_insert(InkDocument::new(String::new(), self.wide_encoding));
                entry.edit(edits);
                Response::Ok
            }
            Command::ForgetDocument(uri) => match self.documents.remove(&uri) {
                Some(_) => Response::Ok,
                None => Response::Error(format!("Document {} not known", uri.path())),
            },
        }
    }
}

pub(crate) fn run(
    mut state: State,
    receiver: Receiver<Request>,
) -> AppResult<std::thread::JoinHandle<()>> {
    let handle = std::thread::Builder::new()
        .name("lsp-state".to_owned())
        .spawn(move || {
            for (command, reply_to) in receiver {
                let response = state.handle(command);
                if let Some(responder) = reply_to {
                    if let Err(e) = responder.send(response) {
                        eprintln!("couldn't send reply: {e}");
                    }
                }
            }
        })?;
    Ok(handle)
}
