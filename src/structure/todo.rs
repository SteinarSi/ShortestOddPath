use std::cmp::Ordering;
use std::cmp::Ordering::{Greater, Less};
use crate::structure::weight::Weight;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Todo<W: Weight> {
    Vertex(W, usize),
    Blossom(W, usize, usize),
}
use Todo::*;

impl <W: Weight> PartialOrd for Todo<W> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl <W: Weight> Ord for Todo<W> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Vertex(w1, _),Vertex(w2, _)) => w1.cmp(w2),
            (Vertex(w1,_),Blossom(w2,_,_)) => if *w1 + *w1 <= *w2 { Less } else { Greater },
            (Blossom(w1,_,_),Vertex(w2,_)) => if *w1 < *w2 + *w2 { Less } else { Greater },
            (Blossom(w1,_,_),Blossom(w2,_,_)) => w1.cmp(w2),
        }
    }
}