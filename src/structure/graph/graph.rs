use crate::structure::graph::edge::Edge;
use crate::structure::weight::Weight;

pub trait Graph<'a, E, W>
    where E: Edge<W>,
          W: Weight,
{
    type V: PartialEq + Clone + 'a;
    fn n(&self) -> usize;
    fn m(&self) -> usize;
    fn vertices(&'a self) -> impl Iterator<Item = Self::V>;
    #[allow(non_snake_case)]
    fn V(&'a self) -> impl Iterator<Item = Self::V> {
        self.vertices()
    }
    fn add_edge(&mut self, e: E);
    fn is_adjacent(&self, u: usize, v: usize) -> bool;
}
