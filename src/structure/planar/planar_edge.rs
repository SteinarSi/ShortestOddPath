use std::str::FromStr;
use crate::structure::edge::{Edge};
use crate::structure::weight::{Weight, Weighted};

pub trait PlanarEdge<W: Weight>: Edge<W> {
    fn left(&self) -> usize;
    fn right(&self) -> usize;
}

#[derive(PartialEq, Clone, Debug)]
pub struct PlanarEdgeImpl<W: Weight> {
    pub from: usize,
    pub to: usize,
    pub weight: W,
    pub left: usize,
    pub right: usize,
}

impl <W: Weight> Weighted<W> for PlanarEdgeImpl<W> {
    fn weight(&self) -> W { self.weight }
}

impl <W: Weight> Edge<W> for PlanarEdgeImpl<W> {
    fn from(&self) -> usize { self.from }
    fn to(&self) -> usize { self.to }
    fn reverse(&self) -> Self {
        PlanarEdgeImpl {
            from: self.to,
            to: self.from,
            weight: self.weight,
            left: self.right,
            right: self.left,
        }
    }
}

impl <W: Weight> PlanarEdge<W> for PlanarEdgeImpl<W> {
    fn left(&self) -> usize { self.left }
    fn right(&self) -> usize { self.right }
}

impl <W: Weight> FromStr for PlanarEdgeImpl<W> {
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

        Ok(PlanarEdgeImpl {
            from,
            to,
            weight: w,
            left,
            right,
        })
    }
}