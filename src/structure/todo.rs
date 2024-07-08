use std::cmp::Ordering;
use std::cmp::Ordering::{Equal};
use crate::structure::weight::Weight;
use Todo::*;
use crate::structure::graph::edge::Edge;

#[derive(Debug, Clone)]
pub enum Todo<W: Weight, E: Edge<W>> {
    Vertex(W, usize),
    Blossom(W, E),
}

impl <W: Weight, E: Edge<W>> PartialOrd for Todo<W,E> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Vertex(w1, _),Vertex(w2, _)) => w1.partial_cmp(w2),
            (Vertex(w1,_),Blossom(w2,_)) => (*w1+*w1, false).partial_cmp(&(*w2,true)),
            (Blossom(w1,_),Vertex(w2,_)) => (*w1,true).partial_cmp(&(*w2+*w2,false)),
            (Blossom(w1,_),Blossom(w2,_)) => w1.partial_cmp(w2),
        }
    }
}

impl<W: Weight, E: Edge<W>> Eq for Todo<W,E> {}

impl<W: Weight, E: Edge<W>> PartialEq<Self> for Todo<W,E> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Vertex(w1,u), Vertex(w2,v)) => w1 == w2 && u == v,
            (Blossom(w1, e1), Blossom(w2, e2)) => w1 == w2 && (e1 == e2 || e1 == &e2.reverse()),
            _ => false,
        }
    }
}

impl <W: Weight, E: Edge<W>> Ord for Todo<W,E> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Equal)
    }
}

#[cfg(test)]
mod test_todo {
    use crate::structure::graph::edge::BasicEdge;
    use crate::structure::todo::Todo;
    use crate::structure::todo::Todo::{Blossom, Vertex};

    fn vertex(w: u64) -> Todo<u64, BasicEdge<u64>> {
        Vertex(w, 0)
    }
    fn blossom(w: u64) -> Todo<u64, BasicEdge<u64>> {
        Blossom(w, BasicEdge::new(0, 0, 0))
    }

    #[test]
    fn test_todo() {
        assert!( vertex(2) < vertex(10) );
        assert!( vertex(2) <= vertex(2) );
        assert!( vertex(5) > vertex(0) );
        assert!( vertex(0) >= vertex(0) );
        
        assert!( vertex(2) < blossom(4) );
        assert!( vertex(3) > blossom(4) );
        assert!( vertex(3) > blossom(5) );
        assert!( vertex(3) < blossom(6) );
        assert!( vertex(3) <= blossom(6) );
        assert!( vertex(0) <= blossom(0) );
        assert!( vertex(0) < blossom(0) );
        
        assert!( blossom(2) < vertex(3) );
        assert!( blossom(2) <= vertex(3) );
        assert!( blossom(4) > vertex(2) );
        assert!( blossom(4) >= vertex(2) );
        assert!( blossom(5) > vertex(2) );
        assert!( blossom(6) > vertex(3) );
        assert!( blossom(0) > vertex(0) );

        assert!( blossom(0) < blossom(2) );
        assert!( blossom(0) <= blossom(0) );
        assert!( blossom(17) > blossom(12) );
        assert!( blossom(12) >= blossom(12) );
    }
}