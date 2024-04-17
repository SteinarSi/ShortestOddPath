use std::str::FromStr;
use crate::structure::graph::edge::{Edge};
use crate::structure::graph::planar::planar_edge::PlanarEdgeImpl;
use crate::structure::weight::{Weight, Weighted};

#[derive(PartialEq, Clone, Debug)]
pub struct PrePlanarEdge<W: Weight> {
    pub from: usize,
    pub to: usize,
    pub weight: W,
    pub left: Option<usize>,
    pub right: Option<usize>,
}

impl <W: Weight> PrePlanarEdge<W> {
    pub fn planarize(self) -> PlanarEdgeImpl<W> {
        PlanarEdgeImpl {
            from: self.from,
            to: self.to,
            weight: self.weight,
            left: self.left.unwrap(),
            right: self.right.unwrap(),
        }
    }
}

impl <W: Weight> Edge<W> for PrePlanarEdge<W> {
    fn from(&self) -> usize { self.from }
    fn to(&self) -> usize { self.to }
    fn reverse(&self) -> Self {
        PrePlanarEdge {
            from: self.to,
            to: self.from,
            weight: self.weight,
            left: self.right,
            right: self.left,
        }
    }

    fn subdivide(&self, middle: usize) -> (Self, Self) {
        (
            PrePlanarEdge {
                from: self.from,
                to: middle,
                weight: self.weight,
                left: self.left,
                right: self.right,
            },
            PrePlanarEdge {
                from: middle,
                to: self.to,
                weight: 0.into(),
                left: self.left,
                right: self.right,
            }
        )
    }
    fn shift_by(&self, offset: i64) -> Self {
        PrePlanarEdge {
            from: (self.from as i64 + offset) as usize,
            to: (self.to as i64 + offset) as usize,
            weight: self.weight,
            left: self.left,
            right: self.right,
        }
    }
}

impl <W: Weight> Eq for PrePlanarEdge<W> {}

impl <W: Weight> Weighted<W> for PrePlanarEdge<W> {
    fn weight(&self) -> W { self.weight }
}

impl <W: Weight> FromStr for PrePlanarEdge<W> {
    type Err = &'static str;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let mut ws = str.split(' ');
        Ok(PrePlanarEdge {
            from: ws.next().ok_or("Could not find the first vertex id")?.parse().or(Err("Could not parse the first vertex id"))?,
            to: ws.next().ok_or("Could not find the second vertex id")?.parse().or(Err("Could not parse the second vertex id"))?,
            weight: ws.next().get_or_insert("1").parse().or(Err("Could not parse the weight"))?,
            left: None,
            right: None,
        })
    }
}