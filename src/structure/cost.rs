use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, Sub};
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cost {
    Infinite,
    Finite(u64)
}
pub use Cost::*;

impl Cost {
    pub fn is_infinite(&self) -> bool {
        match self {
            Infinite => true,
            Finite(_) => false,
        }
    }

    pub fn is_finite(&self) -> bool {
        ! self.is_infinite()
    }

    pub fn unwrap(&self) -> u64 {
        self.expect("Error: tried to unwrap an infinite value and treat it as strictly finite")
    }

    pub fn expect(&self, msg: &str) -> u64 {
        match self {
            Finite(x) => *x,
            Infinite => panic!("{}", msg),
        }
    }
}
impl Ord for Cost {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Infinite, Infinite) => Ordering::Equal,
            (Infinite, Finite(_)) => Ordering::Greater,
            (Finite(_), Infinite) => Ordering::Less,
            (Finite(a), Finite(b)) => a.cmp(b),
        }
    }
}
impl PartialOrd for Cost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Add for Cost {
    type Output = Cost;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Finite(a), Finite(b)) => Finite(a+b),
            _ => Infinite,
        }
    }
}
impl Sub for Cost {
    type Output = Cost;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Finite(a), Finite(b)) => Finite(a-b),
            _ => Infinite,
        }
    }
}

impl From<Option<u64>> for Cost {
    fn from(value: Option<u64>) -> Self {
        match value {
            None => Infinite,
            Some(x) => Finite(x),
        }
    }
}

impl <E> From<Result<u64, E>> for Cost {
    fn from(value: Result<u64, E>) -> Self {
        match value {
            Err(_) => Infinite,
            Ok(x) => Finite(x),
        }
    }
}

impl FromStr for Cost {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.parse().map_or(Infinite, |x| Finite(x)))
    }
}

impl Debug for Cost {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Finite(x) = self {
            write!(f, "{}", x)
        }
        else {
            write!(f, "∞")
        }
    }
}