use std::ops::{Add, Div, Sub};
use std::str::FromStr;
use std::fmt::{Debug, Display};

pub trait Weight:
    Add<Output = Self> +
    Sub<Output = Self> +
    Div<Output = Self> +
    Clone +
    PartialEq +
    FromStr<Err: Display + Debug> +
    From<u32> +
    PartialOrd +
    Copy +
    Default +
    Debug +
    Display +
{}

impl<T> Weight for T
    where
        T:
            Add<Output = T> +
            Sub<Output = T> +
            Div<Output = T> +
            Clone +
            PartialEq +
            FromStr<Err: Display + Debug> +
            From<u32> +
            PartialOrd +
            Copy +
            Default +
            Debug +
            Display,
{}
