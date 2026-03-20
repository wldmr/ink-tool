use enumflags2::{BitFlag, BitFlags};
use indoc::formatdoc;
use rassert::ExpectationChain;
use std::fmt::Debug;

#[derive(derive_more::Display, derive_more::Debug, derive_more::From)]
pub struct Contains<T: BitFlag>(BitFlags<T>);
pub struct Intersects<T: BitFlag>(BitFlags<T>);

impl<T: BitFlag + Debug> rassert::Expectation<BitFlags<T>> for Contains<T> {
    fn test(&self, actual: &BitFlags<T>) -> bool {
        actual.contains(self.0)
    }

    fn message(&self, _expression: &str, actual: &BitFlags<T>) -> String {
        let expected = self.0;
        formatdoc! {"
                expecting
                    actual value  {actual}
                    to contain    {expected}
        "}
    }
}

impl<T: BitFlag + Debug> rassert::Expectation<BitFlags<T>> for Intersects<T> {
    fn test(&self, actual: &BitFlags<T>) -> bool {
        actual.intersects(self.0)
    }

    fn message(&self, _expression: &str, actual: &BitFlags<T>) -> String {
        let expected = self.0;
        formatdoc! {"
                expecting
                    actual value  {actual}
                    to intersect  {expected}
        "}
    }
}

pub trait BitFlagsExpectation<'a, T: BitFlag> {
    fn to_contain(self, expected: impl Into<BitFlags<T>>) -> ExpectationChain<'a, BitFlags<T>>;
    fn to_intersect(self, expected: impl Into<BitFlags<T>>) -> ExpectationChain<'a, BitFlags<T>>;
}

impl<'a, T: BitFlag + Debug> BitFlagsExpectation<'a, T> for ExpectationChain<'a, BitFlags<T>> {
    fn to_contain(self, expected: impl Into<BitFlags<T>>) -> ExpectationChain<'a, BitFlags<T>> {
        self.expecting(Contains(expected.into()))
    }

    fn to_intersect(self, expected: impl Into<BitFlags<T>>) -> ExpectationChain<'a, BitFlags<T>> {
        self.expecting(Intersects(expected.into()))
    }
}
