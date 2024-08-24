///! # A "stretchy" value with upper and lower bounds and a desired value
///!
///! Used to represent whitespace values, where different rules may place constraints on the minimum and maximum
///! allowable whitespace. Additionally, the (optional) desired value will be used if it ends up lying between the
///! bounds.
use std::{
    fmt::{Debug, Write},
    ops::{Add, AddAssign},
};

// We might want to change the underlying type later
type N = u8;

#[derive(Clone, Copy)]
pub(crate) struct Constrained {
    min: N,
    max: N,
    desired: Option<N>,
}

impl Debug for Constrained {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(value) = self.desired {
            f.write_fmt(format_args!("{}", value))?;
        } else {
            f.write_char('?')?;
        }
        if self.min != N::MIN {
            f.write_fmt(format_args!("≥{}", self.min))?;
        }
        if self.max != N::MAX {
            f.write_fmt(format_args!("≤{}", self.max))?;
        }
        Ok(())
    }
}

impl From<N> for Constrained {
    fn from(value: N) -> Self {
        Constrained::desired(value)
    }
}

impl Into<N> for Constrained {
    fn into(self) -> N {
        self.value()
    }
}

impl From<usize> for Constrained {
    fn from(value: usize) -> Self {
        Constrained::desired(value.min(N::MAX as usize) as N)
    }
}

impl PartialEq<Constrained> for Constrained {
    fn eq(&self, other: &Constrained) -> bool {
        self.desired.eq(&other.desired)
    }
}

impl PartialEq<usize> for Constrained {
    fn eq(&self, other: &usize) -> bool {
        if *other < (self.min as usize) || *other > (self.max as usize) {
            return false;
        } else if let Some(desired) = self.desired {
            let other_n = *other as N;
            desired.eq(&other_n)
        } else {
            true
        }
    }
}

impl Eq for Constrained {}

impl PartialOrd<Constrained> for Constrained {
    fn partial_cmp(&self, other: &Constrained) -> Option<std::cmp::Ordering> {
        self.desired.partial_cmp(&other.desired)
    }
}

impl Ord for Constrained {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.desired.cmp(&other.desired)
    }
}

impl Add for Constrained {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.combine(&rhs)
    }
}

impl AddAssign for Constrained {
    fn add_assign(&mut self, rhs: Self) {
        self.combine_mut(rhs)
    }
}

impl Constrained {
    pub(crate) fn new() -> Self {
        Self {
            min: N::MIN,
            max: N::MAX,
            desired: None,
        }
    }

    /// Construct an unconstrained(!) desired value.
    pub(crate) fn desired(value: N) -> Self {
        let mut me = Self::new();
        me.desired = Some(value);
        me
    }

    /// Construct a value of at least this. Does not set a desired value.
    pub(crate) fn at_least(value: N) -> Self {
        let mut me = Self::new();
        me.min = value;
        me
    }

    /// Construct a value of at most this. Does not set a desired value.
    pub(crate) fn at_most(value: N) -> Self {
        let mut me = Self::new();
        me.max = value;
        me
    }

    /// Construct a value between `min` and `max`.
    pub(crate) fn between(min: N, max: N) -> Self {
        Self::at_least(min) + Self::at_most(max)
    }

    /// The value is determind by the following priority:
    ///
    /// 1. The desired value, if it is within the bounds.
    /// 2. If the desired value exceeds the bounds, the bound that it ran up against.
    /// 3. If there is no desired value, the lower bound.
    pub(crate) fn value(&self) -> N {
        self.desired
            .map(|it| it.clamp(self.min, self.max))
            .unwrap_or(self.min)
    }

    pub(crate) fn combine_mut(&mut self, other: Constrained) {
        self.min = Ord::max(self.min, other.min);
        self.max = Ord::min(self.max, other.max);
        if self.min > self.max {
            // ensure that min never exceeds max, by growing max.
            self.max = self.min
        }
        self.desired = match (self.desired, other.desired) {
            (None, None) => None,
            (None, Some(b)) => Some(b),
            (Some(a), None) => Some(a),
            (Some(a), Some(b)) => Some(Ord::max(a, b)),
        }
    }

    pub(crate) fn combine(&self, other: &Constrained) -> Self {
        let mut new = self.clone();
        new.combine_mut(other.clone());
        new
    }
}

#[cfg(test)]
mod tests {
    use super::Constrained as C;
    use super::*;
    use quickcheck::{Arbitrary, TestResult};
    use quickcheck_macros::quickcheck;

    impl Arbitrary for Constrained {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let min = N::arbitrary(g);
            let max = N::arbitrary(g);
            let mut new = Self::between(min, max);
            new.desired = Option::arbitrary(g);
            new
        }
    }

    macro_rules! in_case {
        ($prereq:expr => $($stmts:stmt);+) => {
            if $prereq {
                TestResult::from_bool({$($stmts)*})
            } else {
                TestResult::discard()
            }
        };
    }

    #[quickcheck]
    fn unconstrained_value(n: N) -> bool {
        C::desired(n).value() == n
    }

    #[quickcheck]
    fn value_greater_equal_min(constrained: Constrained) -> bool {
        constrained.value() >= constrained.min
    }

    #[quickcheck]
    fn value_less_equal_min(constrained: Constrained) -> bool {
        constrained.value() <= constrained.max
    }

    #[quickcheck]
    fn value_without_desired_is_min(c: Constrained) -> TestResult {
        in_case! { c.desired.is_none() => c.value() == c.min }
    }

    #[quickcheck]
    fn unconstrained_plus_at_most(n: N, max: N) -> bool {
        let sum = Constrained::desired(n) + Constrained::at_most(max);
        sum.value() == Ord::min(n, max)
    }

    #[quickcheck]
    fn unconstrained_plus_at_least(n: N, min: N) -> bool {
        let sum = Constrained::desired(n) + Constrained::at_least(min);
        sum.value() == Ord::max(n, min)
    }

    #[quickcheck]
    fn plus_desired_is_always_max(a: Constrained, b: Constrained) -> bool {
        let c = a + b;
        c.desired == Ord::max(a.desired, b.desired)
    }

    #[quickcheck]
    fn plus_min_is_biggest_possible(a: Constrained, b: Constrained) -> bool {
        let c = a + b;
        c.min == Ord::max(a.min, b.min)
    }

    #[quickcheck]
    fn plus_max_is_smallest_possible(a: Constrained, b: Constrained) -> TestResult {
        in_case! { a.min <= b.max && b.min <= a.max =>
            let c = a + b;
            c.max == Ord::min(a.max, b.max)
        }
    }

    #[quickcheck]
    /// Conflict resolution: If any min bound is greater than the other max bound,
    /// the min wins and the max grows to accomodate it..
    fn plus_conflict_grows_max(a: Constrained, b: Constrained) -> TestResult {
        in_case! { a.min > b.max || b.min > a.max =>
            let c = a + b;
            let new_max = Ord::max(a.min, b.min);
            c.min == c.max && c.max == new_max
        }
    }
}
