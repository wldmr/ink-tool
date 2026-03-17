use enumflags2::{BitFlag, BitFlags};
use indoc::formatdoc;
use rassert::ExpectationChain;

pub struct IsSubset<T: BitFlag>(BitFlags<T>);
pub struct Intersects<T: BitFlag>(BitFlags<T>);

impl<T: BitFlag + std::fmt::Debug> rassert::Expectation<BitFlags<T>> for IsSubset<T> {
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

impl<T: BitFlag + std::fmt::Debug> rassert::Expectation<BitFlags<T>> for Intersects<T> {
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

impl<'a, T: BitFlag + std::fmt::Debug> BitFlagsExpectation<'a, T>
    for ExpectationChain<'a, BitFlags<T>>
{
    fn to_contain(self, expected: impl Into<BitFlags<T>>) -> ExpectationChain<'a, BitFlags<T>> {
        self.expecting(IsSubset(expected.into()))
    }

    fn to_intersect(self, expected: impl Into<BitFlags<T>>) -> ExpectationChain<'a, BitFlags<T>> {
        self.expecting(Intersects(expected.into()))
    }
}
