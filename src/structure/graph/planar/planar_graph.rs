use std::fmt::{Debug, Formatter};
use std::ops::Index;
use std::str::FromStr;

use crate::structure::graph::edge::{Edge, map_to};
use crate::structure::graph::graph::{Graph, GraphInternal};
use crate::structure::graph::planar::planar_edge::PlanarEdge;
use crate::structure::graph::planar::point::Point;
use crate::structure::graph::planar::pre_planar_graph::PrePlanarGraph;
use crate::structure::graph::undirected_graph::UndirectedGraph;
use crate::structure::weight::Weight;

#[derive(Clone)]
pub struct PlanarGraph<W: Weight> {
    points: Vec<Point>,
    adj_list: Vec<Vec<PlanarEdge<W>>>,
    dual: UndirectedGraph<W, PlanarEdge<W>>,
    m: usize,
}

impl <'a, W: Weight> PlanarGraph<W> {
    pub fn new(points: Vec<Point>, adj_list: Vec<Vec<PlanarEdge<W>>>, dual: UndirectedGraph<W,PlanarEdge<W>>, m: usize) -> Self {
        PlanarGraph {
            points,
            adj_list,
            dual,
            m
        }
    }
    pub fn f(&self) -> usize { self.dual.n() }
    pub fn points(&self, u: usize) -> &Point { &self.points[u] }
    pub fn dual(&self) -> &UndirectedGraph<W,PlanarEdge<W>> { &self.dual }
    #[allow(non_snake_case)]
    pub fn N(&self, u: usize) -> &Vec<PlanarEdge<W>> { &self.adj_list[u] }
    pub fn delete_edges(&mut self, r: &Vec<PlanarEdge<W>>) {
        for e in r {
            self.adj_list[e.from()].retain(|f| f != e);
        }
    }
}
impl <W: Weight> GraphInternal<PlanarEdge<W>,W> for PlanarGraph<W> {
    fn adj_list(&self) -> &Vec<Vec<PlanarEdge<W>>> { &self.adj_list }
    fn adj_list_mut(&mut self) -> &mut Vec<Vec<PlanarEdge<W>>> { &mut self.adj_list }
    fn m_mut(&mut self) -> &mut usize { &mut self.m }
}

impl <'a, W: Weight> Graph<'a, PlanarEdge<W>, W> for PlanarGraph<W> {
    type V = Point;
    fn n(&self) -> usize { self.points.len() }
    fn m(&self) -> usize { self.m }
    fn vertices(&'a self) -> impl Iterator<Item = Point> {
        self.points.clone().into_iter()
    }
}

impl <W: Weight> FromStr for PlanarGraph<W> {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        PrePlanarGraph::from_str(s)?.planarize()
    }
}

impl <W: Weight> Debug for PlanarGraph<W> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut ret = String::new();
        ret.push_str(format!("PlanarGraph(n = {}, m = {}):\n", self.n(), self.m()).as_str());
        for u in self.vertices() {
            ret.push_str(format!("  N({}) = {:?}", u.id, map_to(&self.adj_list[u.id])).as_str());
        }
        write!(f, "{}", ret)
    }
}

impl <W: Weight> Index<&usize> for PlanarGraph<W> {
    type Output = Vec<PlanarEdge<W>>;
    fn index(&self, index: &usize) -> &Self::Output { &self.adj_list[*index] }
}

impl <W: Weight> Index<&Point> for PlanarGraph<W> {
    type Output = Vec<PlanarEdge<W>>;
    fn index(&self, index: &Point) -> &Self::Output { &self.adj_list[index.id] }
}


mod test_planar_graph {
    use crate::structure::graph::graph::Graph;

    #[test]
    fn test_small_planar1() {
        let planar: super::PlanarGraph <f64> = std::fs::read_to_string("data/planar_graphs/small_planar1/small_planar1.in")
            .expect("No graph found")
            .parse()
            .expect("Could not parse graph");

        assert!(planar.dual().is_adjacent(0, 0));
        assert!(planar.dual().is_adjacent(0, 1));
        assert!(planar.dual().is_adjacent(1, 3));
        assert!(planar.dual().is_adjacent(1, 2));
        assert!(planar.dual().is_adjacent(2, 4));
        assert!(planar.dual().is_adjacent(3, 4));
        assert!(planar.dual().is_adjacent(3, 5));
        assert!( ! planar.dual().is_adjacent(2, 5));
        assert!( ! planar.dual().is_adjacent(2, 6));
        assert!( ! planar.dual().is_adjacent(1, 5));
        assert!( ! planar.dual().is_adjacent(2, 6));

        println!("{:?}", planar);
    }
}







