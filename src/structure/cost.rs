use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, Sub};
use std::str::FromStr;
pub use Cost::*;
use crate::structure::weight::Weight;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cost<W: Weight> {
    Infinite,
    Finite(W),
}

impl <W: Weight> Cost<W> {
    pub fn is_infinite(&self) -> bool {
        match self {
            Infinite => true,
            Finite(_) => false,
        }
    }

    pub fn is_finite(&self) -> bool {
        ! self.is_infinite()
    }

    pub fn unwrap(&self) -> W {
        self.expect("Error: tried to unwrap an infinite value and treat it as strictly finite")
    }

    pub fn expect(&self, msg: &str) -> W {
        match self {
            Finite(x) => *x,
            Infinite => panic!("{}", msg),
        }
    }
}

impl <W: Weight> PartialOrd for Cost<W> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Infinite, Infinite) => Some(Ordering::Equal),
            (Infinite, Finite(_)) => Some(Ordering::Greater),
            (Finite(_), Infinite) => Some(Ordering::Less),
            (Finite(a), Finite(b)) => a.partial_cmp(b),
        }
    }
}

impl <W: Weight> Add for Cost<W> {
    type Output = Cost<W>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Finite(a), Finite(b)) => Finite(a+b),
            _ => Infinite,
        }
    }
}
impl <W: Weight> Sub for Cost<W> {
    type Output = Cost<W>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Finite(a), Finite(b)) => Finite(a-b),
            _ => Infinite,
        }
    }
}

impl <W: Weight> From<Option<u32>> for Cost<W> {
    fn from(value: Option<u32>) -> Self {
        match value {
            None => Infinite,
            Some(x) => Finite(W::from(x)),
        }
    }
}

impl <E, W: Weight> From<Result<u32, E>> for Cost<W> {
    fn from(value: Result<u32, E>) -> Self {
        match value {
            Err(_) => Infinite,
            Ok(x) => Finite(W::from(x)),
        }
    }
}

impl <W: Weight> FromStr for Cost<W> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.parse().map_or(Infinite, |x| Finite(x)))
    }
}

impl <W: Weight> Debug for Cost<W> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Finite(x) = self {
            write!(f, "{}", x)
        }
        else {
            write!(f, "∞")
        }
    }
}