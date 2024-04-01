use std::cmp::Ordering;
use std::cmp::Ordering::{Equal};
use crate::structure::weight::Weight;

#[derive(Debug, Clone)]
pub enum Todo<W: Weight> {
    Vertex(W, usize),
    Blossom(W, usize, usize),
}
use Todo::*;

impl <W: Weight> PartialOrd for Todo<W> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Vertex(w1, _),Vertex(w2, _)) => w1.partial_cmp(w2),
            (Vertex(w1,_),Blossom(w2,_,_)) => (*w1+*w1).partial_cmp(w2),
            (Blossom(w1,_,_),Vertex(w2,_)) => (*w1).partial_cmp(&(*w2+*w2)),
            (Blossom(w1,_,_),Blossom(w2,_,_)) => w1.partial_cmp(w2),
        }
    }
}

impl<W: Weight> Eq for Todo<W> {}

impl<W: Weight> PartialEq<Self> for Todo<W> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Vertex(w1,u), Vertex(w2,v)) => w1 == w2 && u == v,
            (Blossom(w1, u, v), Blossom(w2, a, b)) => w1 == w2 && ((u == a && v == b) || (u == b && v == a)),
            _ => false,
        }
    }
}

impl <W: Weight> Ord for Todo<W> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Equal)
    }
}