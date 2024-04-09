use std::str::FromStr;
use crate::structure::planar::line::Line;
use crate::structure::weight::Weight;

#[derive(PartialEq, Clone)]
pub struct PrePlanarLine<W: Weight> {
    pub from: usize,
    pub to: usize,
    pub weight: W,
    pub left: Option<usize>,
    pub right: Option<usize>,
}

impl <W: Weight> PrePlanarLine<W> {
    pub fn reverse(&self) -> Self {
        PrePlanarLine {
            from: self.to,
            to: self.from,
            weight: self.weight,
            left: self.right,
            right: self.left,
        }
    }
    pub fn planarize(self) -> Line<W> {
        Line {
            from: self.from,
            to: self.to,
            weight: self.weight,
            left: self.left.unwrap(),
            right: self.right.unwrap(),
        }
    }
}

impl <W: Weight> FromStr for PrePlanarLine<W> {
    type Err = &'static str;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let mut ws = str.split(' ');
        Ok(PrePlanarLine {
            from: ws.next().ok_or("Could not find the first vertex id")?.parse().or(Err("Could not parse the first vertex id"))?,
            to: ws.next().ok_or("Could not find the second vertex id")?.parse().or(Err("Could not parse the second vertex id"))?,
            weight: ws.next().get_or_insert("1").parse().or(Err("Could not parse the weight"))?,
            left: None,
            right: None,
        })
    }
}