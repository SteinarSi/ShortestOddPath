use std::fmt::Debug;
use std::str::FromStr;
use crate::structure::weight::{Weight, Weighted};

pub trait Edge<W: Weight>: Weighted<W> + FromStr + Debug + Clone {
    fn from(&self) -> usize;
    fn to(&self) -> usize;
    fn reverse(&self) -> Self;
    fn subdivide(&self, middle: usize) -> (Self, Self);
}

#[derive(PartialEq, Clone, Debug)]
pub struct BasicEdge<W: Weight> {
    from: usize,
    to: usize,
    weight: W,
}

impl <W: Weight> BasicEdge<W> {
    pub fn new(from: usize, to: usize, weight: W) -> Self {
        BasicEdge {
            from,
            to,
            weight,
        }
    }
}

impl <W: Weight> Edge<W> for BasicEdge<W> {
    fn from(&self) -> usize { self.from }
    fn to(&self) -> usize { self.to }
    fn reverse(&self) -> Self {
        BasicEdge {
            from: self.to,
            to: self.from,
            weight: self.weight,
        }
    }

    fn subdivide(&self, middle: usize) -> (Self, Self) {
        (
            BasicEdge {
                from: self.from,
                to: middle,
                weight: 0.into(),
            },
            BasicEdge {
                from: middle,
                to: self.to,
                weight: 0.into(),
            }
        )
    }
}

impl <W: Weight> Weighted<W> for BasicEdge<W> {
    fn weight(&self) -> W { self.weight }
}

impl <W: Weight> FromStr for BasicEdge<W> {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rs = s.split(' ');
        let u = rs.next()
            .ok_or("Expected an unsigned integer here, but found nothing!")?
            .parse()
            .or(Err("Could not parse as an unsigned integer!"))?;
        let v = rs.next()
            .ok_or("Expected an unsigned integer here, but found nothing!")?
            .parse()
            .or(Err("Could not parse as an unsigned integer!"))?;
        let w = W::from_str(rs.next().unwrap_or_else(|| "1")).unwrap_or_else(|_|1.into());

        Ok(BasicEdge {
            from: u,
            to: v,
            weight: w,
        })
    }
}