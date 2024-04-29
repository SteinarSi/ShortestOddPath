use std::cmp::Ordering;
use std::cmp::Ordering::Equal;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use crate::structure::graph::edge::{Edge};
use crate::structure::weight::{Weight, Weighted};

#[derive(PartialEq, Clone)]
pub struct PlanarEdge<W: Weight> {
    pub from: usize,
    pub to: usize,
    pub weight: W,
    pub left: usize,
    pub right: usize,
}

impl <W: Weight> PlanarEdge<W> {
    pub fn left(&self) -> usize { self.left }
    pub fn right(&self) -> usize { self.right }
    pub fn rotate_right(&self) -> Self {
        PlanarEdge {
            from: self.left,
            to: self.right,
            left: self.to,
            right: self.from,
            weight: self.weight,
        }
    }
}

impl <W: Weight> Weighted<W> for PlanarEdge<W> {
    fn weight(&self) -> W { self.weight }
}

impl<W: Weight> Debug for PlanarEdge<W> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -{}-> {}", self.from, self.weight, self.to)
    }
}

impl<W: Weight> PartialOrd for PlanarEdge<W> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (self.from,self.to,self.left,self.right,self.weight).partial_cmp(&(other.from,other.to,other.left,other.right,other.weight))
    }
}

impl<W: Weight> Ord for PlanarEdge<W> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Equal)
    }
}

impl <W: Weight> Edge<W> for PlanarEdge<W> {
    fn from(&self) -> usize { self.from }
    fn to(&self) -> usize { self.to }
    fn reverse(&self) -> Self {
        PlanarEdge {
            from: self.to,
            to: self.from,
            weight: self.weight,
            left: self.right,
            right: self.left,
        }
    }

    fn subdivide(&self, middle: usize) -> (Self, Self) {
        (
            PlanarEdge {
                from: self.from,
                to: middle,
                weight: self.weight,
                left: self.left,
                right: self.right,
            },
            PlanarEdge {
                from: middle,
                to: self.to,
                weight: 0.into(),
                left: self.left,
                right: self.right,
            }
        )
    }
    fn shift_by(&self, offset: i64) -> Self {
        PlanarEdge {
            from: (self.from as i64 + offset) as usize,
            to: (self.to as i64 + offset) as usize,
            weight: self.weight,
            left: self.left,
            right: self.right,
        }
    }
}

impl <W: Weight> Eq for PlanarEdge<W> {}

impl <W: Weight> FromStr for PlanarEdge<W> {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rs = s.split(' ');
        let from = rs.next()
            .ok_or("Expected an unsigned integer here, but found nothing!")?
            .parse()
            .or(Err("Could not parse as an unsigned integer!"))?;
        let to = rs.next()
            .ok_or("Expected an unsigned integer here, but found nothing!")?
            .parse()
            .or(Err("Could not parse as an unsigned integer!"))?;
        let left = rs.next()
            .ok_or("Expected an unsigned integer here, but found nothing!")?
            .parse()
            .or(Err("Could not parse as an unsigned integer!"))?;
        let right = rs.next()
            .ok_or("Expected an unsigned integer here, but found nothing!")?
            .parse()
            .or(Err("Could not parse as an unsigned integer!"))?;
        let w = W::from_str(rs.next().unwrap_or_else(|| "1")).unwrap_or_else(|_|1.into());

        Ok(PlanarEdge {
            from,
            to,
            weight: w,
            left,
            right,
        })
    }
}