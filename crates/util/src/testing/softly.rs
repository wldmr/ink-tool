use std::{any::Any, cell::RefCell, ops::ControlFlow, panic::UnwindSafe};

thread_local! {
    /// We store all panic payloads in a thread local variable. The first one to be
    /// dropped cleans this out completely and re-raises a panic. All the following
    /// guards will find this empty, so they won’t panic.
    ///
    /// This means that there will be no panics-inside-panics, so no aborts (😬 … I
    /// hope). It also means that errors aren’t guard specific, but that’s probably a
    /// good thing for simplicity.
    static SOFT_PANICS: RefCell<Vec<AnyPanic>> = Vec::new().into();
}

/// The main way to use soft assertions. Takes an expression that panics on error.
/// The panic is deferred until the end of the current scope.
///
/// When given arguments, creates an “anonyoums” [`SoftGuard`].
///
/// ``` rust
/// # #[macro_use] extern crate util;
/// softly!(
///    assert_eq!(1, 1),
///    assert_eq!("no", "no")
/// );
/// ```
///
/// If you want to create a named guard, use this macro witout any arguments.
///
/// ``` rust
/// # #[macro_use] extern crate util;
/// let guard = softly!();
/// let c1 = guard.check(assert_eq!(1, 1));
/// let c2 = guard.check(assert_eq!("no", "no"));
/// assert!(c1.is_ok());
/// assert!(c2.is_ok());
/// ```
#[macro_export]
macro_rules! softly {

    () => {
        $crate::testing::softly::SoftGuard::new()
    };

    ($($expr:expr),+$(,)?) => {
        let _guard = $crate::softly!();
        $( _guard.run(|| $expr ); )+
    };
}

/// Capture the output of each check in a variable.
///
/// ```rust
/// # #[macro_use] extern crate util;
/// softly_let!(
///    a = assert_eq!(1, 1),
///    b = assert_eq!("no", "no")
/// );
/// softly!(
///    assert!(a.is_ok()),
///    assert!(b.is_ok()),
/// );
/// ```
#[macro_export]
macro_rules! softly_let {
    ($($ident:ident = $expr:expr),+$(,)?) => {
        let _guard = $crate::softly!();
        $( let $ident = _guard.check(|| $expr); )+
    };
}

/// Creates a new guard. When it goes out of scope, and any errors have ocurred,
/// raise a panic with all errors so far.
impl SoftGuard {
    pub fn new() -> SoftGuard {
        SoftGuard(Private)
    }
}

type AnyPanic = Box<dyn Any + Send + 'static>;

/// Defers all panics encountered
///
/// The first guard that goes out of scope after any failed soft assertion
/// will trigger a panic (i.e. test failure).
#[must_use = "If you don't assign this to a variable, \
then it'll go out of scope right here, \
possibly ending the test early."]
pub struct SoftGuard(Private);
// So that it can't be constructed without our factory function.
struct Private;

impl SoftGuard {
    /// Run a soft check on `p`.
    pub fn check<P: PanicOnFailure + UnwindSafe>(&self, p: P) -> Result<P::Output, ()> {
        let result = std::panic::catch_unwind(|| p.panic_on_failure());
        match result {
            Ok(ok) => Ok(ok),
            Err(err) => {
                SOFT_PANICS.with_borrow_mut(|collect| collect.push(err));
                Err(())
            }
        }
    }

    /// Like [`Self::check`] but without a return value
    ///
    /// If you don't want to inspect the values, and want to avoid the `must_use` warnings.
    pub fn run<P: PanicOnFailure + UnwindSafe>(&self, p: P) {
        _ = self.check(p);
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
        if std::thread::panicking() {
            return; // No need to aggravate things.
        }
        let panics = SOFT_PANICS.take();
        if !panics.is_empty() {
            std::panic::resume_unwind(Box::new(panics));
        }
    }
}

/// Implement this trait if have a custom type that you want to use for testing.
///
/// (You probably don’t need to do this. If your testing library has a panicking
/// function/macro (akin to [`std::assert`]), then just use [`softly!`] on
/// that).
pub trait PanicOnFailure {
    type Output: PanicOnFailure;
    /// Does its checks, and then panics on failure.
    fn panic_on_failure(self) -> Self::Output;
}

/// The party trick of this library: This function calls itself, and maybe panics.
/// If it doesn’t panic, then it calls `panic_on_failure()` on the value it
/// produced.
///
/// This means that you can wrap any failure-carrying operation into a closure, and
/// it’ll keep unwrapping itself until either a panic, or success.
impl<'a, F, T> PanicOnFailure for F
where
    F: FnOnce() -> T,
    T: PanicOnFailure,
{
    type Output = T::Output;
    fn panic_on_failure(self) -> T::Output {
        let value = self();
        value.panic_on_failure()
    }
}

/// The stopping point for most assertions: The unit value. If an expression
/// produces a (), it hasn’t panicked.
impl PanicOnFailure for () {
    type Output = ();
    fn panic_on_failure(self) {}
}

/// Panics on [`Err`]
impl<T, E> PanicOnFailure for Result<T, E>
where
    E: std::fmt::Debug,
{
    type Output = ();
    fn panic_on_failure(self) {
        if let Err(err) = self {
            panic!("{err:?}")
        }
    }
}

/// Panics on [`ControlFlow::Break`]
impl<B, C> PanicOnFailure for ControlFlow<B, C>
where
    B: std::fmt::Debug,
{
    type Output = ();
    fn panic_on_failure(self) {
        use std::ops::ControlFlow::*;
        if let Break(msg) = self {
            panic!("{msg:?}")
        }
    }
}

/// Panics on `false`
impl PanicOnFailure for bool {
    type Output = ();
    fn panic_on_failure(self) {
        if !self {
            panic!("bool was false")
        }
    }
}

impl<T> PanicOnFailure for rassert::ExpectationChain<'_, T> {
    type Output = ();
    fn panic_on_failure(self) {
        self.conclude_panic();
    }
}

/// Synonym for [`SoftGuard::run`]. Saves you a pair of parens, but requires the
/// guard to be marked `mut`.
impl<P: PanicOnFailure + UnwindSafe> std::ops::AddAssign<P> for SoftGuard {
    fn add_assign(&mut self, rhs: P) {
        self.run(rhs);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::{assert, check};
    use pretty_assertions::assert_str_eq;
    use rassert::prelude::*;

    fn count_panics() -> usize {
        SOFT_PANICS.with_borrow(|it| it.len())
    }

    fn discard_panics() {
        SOFT_PANICS.take();
    }

    #[test]
    fn can_check_softly() {
        softly!(assert_eq!(5, 3));
        softly!(assert!(Some("thing") == None));
        softly!(assert_str_eq!("this", "that"), expect!(&1).to_be(&2),);
        softly!(expect!(&"this").to_equal(&"that"));
        softly!(expect!(&"hey").to_equal(&"ho"));

        check!(count_panics() == 6);
        discard_panics();
    }

    #[test]
    fn explicit_guards_and_fail_fast() {
        softly_let!(
            result = {
                let guard1 = softly!();
                guard1.run(|| assert!(Some("thing") == None));
                guard1.run(|| assert_str_eq!("this", "that"));
                guard1.fail_fast();

                let mut guard2 = softly!();
                guard2 += || assert_eq!("ignored", "because of fail_fast");
                guard2 += || expect!(&1).to_be(&2);
            }
        );
        check!(result.is_err());
        discard_panics();
    }
}
