use crate::structure::graph::edge::Edge;
use crate::structure::weight::Weight;

pub trait Graph<'a, E, W>: GraphInternal<E,W>
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
    #[allow(non_snake_case)]
    fn N(&self, u: usize) -> &Vec<E> { &self.adj_list()[u] }
    fn add_edge(&mut self, e: E) {
        let b = e.reverse();
        self.adj_list_mut()[e.from()].push(e);
        self.adj_list_mut()[b.from()].push(b);
        *self.m_mut() += 1;
    }
    fn is_adjacent(&self, u: usize, v: usize) -> bool {
        let (p, q) = if self.adj_list()[u].len() < self.adj_list()[v].len() {
            (u, v)
        }
        else {
            (v, u)
        };
        self.adj_list()[p].iter().find(|e| e.to() == q).is_some()
    }
    fn find_edges(&self, u: usize, v: usize) -> Vec<E> {
        self.adj_list()[u]
            .clone()
            .into_iter()
            .filter(|e| e.to() == v)
            .collect()
    }
}

pub (in crate::structure::graph) trait GraphInternal<E, W>
    where W: Weight,
          E: Edge<W>,
{
    fn adj_list(&self) -> &Vec<Vec<E>>;
    fn adj_list_mut(&mut self) -> &mut Vec<Vec<E>>;
    fn m_mut(&mut self) -> &mut usize;
}
