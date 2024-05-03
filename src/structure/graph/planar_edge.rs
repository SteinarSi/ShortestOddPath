use std::cmp::Ordering;
use std::cmp::Ordering::Equal;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use crate::structure::graph::edge::{Edge};
use crate::structure::weight::{Weight, Weighted};

#[derive(PartialEq, Clone)]
pub struct AbstractPlanarEdge<W: Weight, S: Sealed> {
    pub from: usize,
    pub to: usize,
    pub weight: W,
    pub (in crate::structure::graph) left: S,
    pub (in crate::structure::graph) right: S,
}

trait Sealed: PartialEq + PartialOrd + Copy + Default {}
impl Sealed for usize {}
impl Sealed for Option<usize> {}

pub type PlanarEdge<W> = AbstractPlanarEdge<W, usize>;
pub (in crate::structure::graph) type PrePlanarEdge<W> = AbstractPlanarEdge<W, Option<usize>>;

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

impl <W: Weight> PrePlanarEdge<W> {
    pub fn planarize(&self) -> PlanarEdge<W> {
        PlanarEdge {
            from: self.from,
            to: self.to,
            weight: self.weight,
            left: self.left.unwrap(),
            right: self.right.unwrap(),
        }
    }
}

impl <W: Weight, S: Sealed> Weighted<W> for AbstractPlanarEdge<W,S> {
    fn weight(&self) -> W { self.weight }
}

impl<W: Weight, S: Sealed> Debug for AbstractPlanarEdge<W,S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -{}-> {}", self.from, self.weight, self.to)
    }
}

impl<W: Weight, S: Sealed> PartialOrd for AbstractPlanarEdge<W,S> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (self.from,self.to,&self.left,&self.right,self.weight).partial_cmp(&(other.from,other.to,&other.left,&other.right,other.weight))
    }
}

impl<W: Weight, S: Sealed> Ord for AbstractPlanarEdge<W, S> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Equal)
    }
}

impl <W: Weight, S: Sealed> Edge<W> for AbstractPlanarEdge<W,S> {
    fn from(&self) -> usize { self.from }
    fn to(&self) -> usize { self.to }
    fn reverse(&self) -> Self {
        Self {
            from: self.to,
            to: self.from,
            weight: self.weight,
            left: self.right,
            right: self.left,
        }
    }

    fn subdivide(&self, middle: usize) -> (Self, Self) {
        (
            Self {
                from: self.from,
                to: middle,
                weight: self.weight,
                left: self.left,
                right: self.right,
            },
            Self {
                from: middle,
                to: self.to,
                weight: 0.into(),
                left: self.left,
                right: self.right,
            }
        )
    }
    fn shift_by(&self, offset: i64) -> Self {
        Self {
            from: (self.from as i64 + offset) as usize,
            to: (self.to as i64 + offset) as usize,
            weight: self.weight,
            left: self.left,
            right: self.right,
        }
    }
}

impl <W: Weight, S: Sealed> Eq for AbstractPlanarEdge<W,S> {}

impl <W: Weight, S: Sealed> FromStr for AbstractPlanarEdge<W,S> {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rs = s.split(' ');
        Ok(Self {
            from: rs.next()
                .ok_or("Expected an unsigned integer here, but found nothing!")?
                .parse()
                .or(Err("Could not parse the base of the edge as an unsigned integer!"))?,
            to: rs.next()
                .ok_or("Expected an unsigned integer here, but found nothing!")?
                .parse()
                .or(Err("Could not parse the tip of the edge as an unsigned integer!"))?,
            weight: W::from_str(rs.next().unwrap_or_else(|| "1")).unwrap_or_else(|_|1.into()),
            left: S::default(),
            right: S::default(),
        })
    }
}