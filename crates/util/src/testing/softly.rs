use std::{any::Any, cell::RefCell, marker::PhantomData, panic::UnwindSafe};

thread_local! {
    /// We store all panic payloads in a thread local variable. The first one to be
    /// dropped cleans this out completely re-raises a panic. All the following guards
    /// will find this empty, so they won’t panic.
    ///
    /// This means that there will be no panics-inside-panics, so no aborts (😬 … I
    /// hope). It also means that errors aren’t guard specific, but that’s probably a
    /// good thing for simplicity.
    static SOFT_PANICS: RefCell<Vec<AnyPanic>> = Vec::new().into();
}

/// The main way to use soft assertions. Takes an expression that panics on error.
/// The panic is deferred until the end of the current scope.
///
/// This creates an `anonyoums` [`SoftGuard`]. If you want to create a named
/// guard, use the `soft_guard` function.
#[macro_export]
macro_rules! softly {
    ($($expr:expr),*$(,)?) => {
        let _guard = $crate::testing::softly::soft_guard();
        $( _guard.check(|| $expr); )*
    };
}

#[macro_export]
macro_rules! soft {
    ($($expr:expr),*$(,)?) => {
        let _guard = $crate::testing::softly::soft_guard();
        $( _guard.check($expr); )*
    };
}

/// Creates a new guard. The first guard to
pub fn soft_guard() -> SoftGuard {
    SoftGuard {
        _private: PhantomData,
    }
}

type AnyPanic = Box<dyn Any + Send + 'static>;

/// Deferrs all panics encountered
///
/// The first guard that goes out of scope after any guard has encountered an error
/// will trigger a panic/test failure.
pub struct SoftGuard {
    // So that it can't be constructed without our factory function.
    _private: PhantomData<()>,
}

impl SoftGuard {
    pub fn check(&self, f: impl PanicOnFailure) {
        if let Err(err) = std::panic::catch_unwind(|| f.panic_on_failure()) {
            SOFT_PANICS.with_borrow_mut(|collect| collect.push(err))
        };
    }

    /// Panics if any errors have occurred yet.
    ///
    /// Same as calling [`drop()`] on the soft guard, but reads a bit better.
    pub fn fail_fast(self) {
        drop(self)
    }
}

impl Drop for SoftGuard {
    fn drop(&mut self) {
        let panics = SOFT_PANICS.take();
        if !panics.is_empty() {
            std::panic::resume_unwind(Box::new(panics));
        }
    }
}

/// Implement this trait if have a custom type that you want to use for testing.
///
/// (You probably don’t need to do this. If your testing library has a panicking
/// `FnOnce() -> ()` (such as [`std::assert`]), then just use [`softly!`] on
/// that).
pub trait PanicOnFailure: UnwindSafe {
    /// Does its checks, and then panics on failure.
    fn panic_on_failure(self);
}

impl<F: FnOnce() + UnwindSafe> PanicOnFailure for F {
    fn panic_on_failure(self) {
        self()
    }
}

impl<T, E> PanicOnFailure for Result<T, E>
where
    T: UnwindSafe,
    E: ToString + UnwindSafe,
{
    fn panic_on_failure(self) {
        if let Err(msg) = self {
            panic!("{}", msg.to_string())
        }
    }
}

impl<B, E> PanicOnFailure for std::ops::ControlFlow<B, E>
where
    B: ToString + UnwindSafe,
    E: UnwindSafe,
{
    fn panic_on_failure(self) {
        if let std::ops::ControlFlow::Break(msg) = self {
            panic!("{}", msg.to_string())
        }
    }
}

/// Synonym for check. Saves you a pair of parens, but requires the guard to be marked `mut`.
impl<P: PanicOnFailure> std::ops::AddAssign<P> for SoftGuard {
    fn add_assign(&mut self, rhs: P) {
        self.check(rhs);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::assert;
    use pretty_assertions::assert_str_eq;
    use rassert::prelude::*;

    #[test]
    #[should_panic]
    fn can_check_softly() {
        softly!(assert_eq!(5, 3));
        softly!(assert!(Some("thing") == None));
        softly!(
            assert_str_eq!("this", "that"),
            expect!(&1).to_be(&2).conclude_panic(),
        );
        softly!(expect!(&"this").to_equal(&"that").conclude_panic());
        soft!(expect!(&"hey").to_equal(&"ho").conclude_result());
    }

    #[test]
    #[should_panic]
    fn explicit_guards_and_fail_fast() {
        let guard1 = soft_guard();
        guard1.check(|| assert!(Some("thing") == None));
        guard1.check(|| assert_str_eq!("this", "that"));
        guard1.fail_fast();

        let mut guard2 = soft_guard();
        guard2 += || assert_eq!("ignored", "because of fail_fast");
        guard2 += || expect!(&1).to_be(&2).conclude_panic();
    }
}
