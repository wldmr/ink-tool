use std::{
    any::Any,
    ops::DerefMut as _,
    panic::UnwindSafe,
    sync::{LazyLock, Mutex},
};

type AnyPanic = Box<dyn Any + Send + 'static>;

/// Stores all panics encountered; resumes on the combined panics on drop.
pub struct DelayPanics(&'static LazyLock<Mutex<Vec<AnyPanic>>>);

impl Default for DelayPanics {
    fn default() -> Self {
        static ERRORS: LazyLock<Mutex<Vec<AnyPanic>>> = LazyLock::new(|| Vec::new().into());
        Self(&ERRORS)
    }
}

impl DelayPanics {
    pub fn run(&mut self, f: impl FnOnce() + UnwindSafe) {
        match std::panic::catch_unwind(f) {
            Ok(_) => {}
            Err(err) => self.add_err(err),
        };
    }

    fn add_err(&mut self, err: AnyPanic) {
        self.0
            .lock()
            .expect("no other lock holder must have panicked")
            .push(err);
    }
}

impl Drop for DelayPanics {
    fn drop(&mut self) {
        let mut panics = match self.0.lock() {
            Ok(ok) => ok,
            Err(err) => {
                // Trying not a panic here, but something has gone wrong!
                eprintln!("Error getting lock on drop: {err:?}");
                return;
            }
        };

        if !panics.is_empty() {
            // Unwind with all collected panics at once.
            let vec = std::mem::take(panics.deref_mut());
            drop(panics); // so the other drops don't panic;
            std::panic::resume_unwind(Box::new(vec));
        }
    }
}

#[macro_export]
macro_rules! softly {
        ($($expr:expr),+$(,)?) => {
            let mut _guard = $crate::testing::delay_panics::DelayPanics::default();
            $(
                _guard.run(|| $expr);
            )+
        };
    }
