use std::collections::BTreeSet;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;

use crate::structure::graph::edge::Edge;
use crate::structure::graph::graph::Graph;
use crate::structure::graph::planar::planar_edge::PlanarEdge;
use crate::structure::graph::planar::point::Point;
use crate::structure::graph::planar::pre_planar_graph::PrePlanarGraph;
use crate::structure::graph::undirected_graph::UndirectedGraph;
use crate::structure::weight::Weight;

#[derive(Clone)]
pub struct PlanarGraph<W: Weight> {
    points: Vec<Point>,
    lines: Vec<PlanarEdge<W>>,
    adj_list: Vec<Vec<usize>>,
    dual: UndirectedGraph<W, PlanarEdge<W>>,
}

impl <'a, W: Weight> PlanarGraph<W> {
    pub fn new(points: Vec<Point>, lines: Vec<PlanarEdge<W>>, adj_list: Vec<Vec<usize>>, dual: UndirectedGraph<W,PlanarEdge<W>>) -> Self {
        PlanarGraph {
            points,
            lines,
            adj_list,
            dual,
        }
    }
    pub fn f(&self) -> usize {
        self.dual.n()
    }
    pub fn points(&self, u: usize) -> &Point {
        &self.points[u]
    }
    pub fn dual(&self) -> &UndirectedGraph<W,PlanarEdge<W>> {
        &self.dual
    }
    #[allow(non_snake_case)]
    pub fn N(&self, u: usize) -> Vec<PlanarEdge<W>> {
        self.adj_list[u]
            .iter()
            .map(|v| self.lines[*v].clone())
            .collect()
    }
    pub fn delete_edges(&self, r: &Vec<PlanarEdge<W>>) -> UndirectedGraph<W,PlanarEdge<W>> {
        let mut x = BTreeSet::new();
        for e in r {
            x.insert(e.clone());
        }
        let mut ret = UndirectedGraph::new(self.n());
        for e in &self.lines {
            if ! x.contains(&e) && ! x.contains(&e.reverse()) {
                ret.add_edge(e.clone());
            }
        }
        ret
    }
}

impl <'a, W: Weight> Graph<'a, PlanarEdge<W>, W> for PlanarGraph<W> {
    type V = Point;
    fn n(&self) -> usize { self.points.len() }
    fn m(&self) -> usize { self.lines.len() / 2 }
    fn vertices(&'a self) -> impl Iterator<Item = Point> {
        self.points.clone().into_iter()
    }
    fn N(&self, u: usize) -> Vec<PlanarEdge<W>> {
        self.adj_list[u].iter().map(|i| self.lines[*i].clone()).collect()
    }

    fn add_edge(&mut self, e: PlanarEdge<W>) {
        let b = e.reverse();
        self.adj_list[e.from].push(self.lines.len());
        self.lines.push(e);
        self.adj_list[b.from].push(self.lines.len());
        self.lines.push(b);
    }

    fn is_adjacent(&self, u: usize, v: usize) -> bool {
        self.adj_list[u].iter().find(|e|self.lines[**e].to == v).is_some()
    }

    fn find_edges(&self, u: usize, v: usize) -> Vec<PlanarEdge<W>> {
        self.adj_list[u].iter().map(|i| self.lines[*i].clone()).filter(|e| e.to() == v).collect()
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
            ret.push_str(format!("  N({}) = {:?}\n", u.id, self.adj_list[u.id].iter().map(|p| {
                self.lines[*p].to
            }).collect::<Vec<usize>>()).as_str())
        }
        write!(f, "{}", ret)
    }
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







