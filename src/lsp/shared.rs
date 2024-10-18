use lsp_server::ResponseError;
use std::{
    ops::DerefMut,
    sync::{Arc, Mutex},
};

/// Newtype around a shared reference, in order to get nicer error handling using `?`.
pub(crate) struct SharedValue<T>(Arc<Mutex<T>>);

impl<T> Clone for SharedValue<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub(crate) struct SharedValueError(String);

impl<T> SharedValue<T> {
    pub(crate) fn new(t: T) -> Self {
        Self(Arc::new(Mutex::new(t)))
    }

    pub(crate) fn lock(&self) -> Result<impl DerefMut<Target = T> + '_, SharedValueError> {
        self.0.lock().map_err(|e| SharedValueError(e.to_string()))
    }
}

impl From<SharedValueError> for ResponseError {
    fn from(value: SharedValueError) -> Self {
        Self {
            code: lsp_server::ErrorCode::InternalError as i32,
            message: value.0,
            data: None,
        }
    }
}
