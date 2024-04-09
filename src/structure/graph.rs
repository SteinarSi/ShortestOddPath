use crate::structure::edge::Edge;
use crate::structure::weight::Weight;

pub trait Graph<'a, V, E, W>
    where V: PartialEq + Clone + 'a,
          E: Edge<W>,
          W: Weight,
{
    fn n(&self) -> usize;
    fn m(&self) -> usize;
    fn vertices(&'a self) -> impl Iterator<Item = V>;
    #[allow(non_snake_case)]
    fn V(&'a self) -> impl Iterator<Item = V> {
        self.vertices()
    }
    fn add_edge(&mut self, e: E);
}
