use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use crate::structure::graph::Graph;
use crate::structure::planar::line::Line;
use crate::structure::planar::point::Point;
use crate::structure::planar::pre_planar_graph::PrePlanarGraph;
use crate::structure::undirected_graph::UndirectedGraph;
use crate::structure::weight::Weight;

pub struct PlanarGraph<W: Weight> {
    points: Vec<Point>,
    lines: Vec<Line<W>>,
    adj_list: Vec<Vec<usize>>,
    dual: UndirectedGraph<W>,
    m: usize,
}

impl <'a, W: Weight> PlanarGraph<W> {
    pub fn new(points: Vec<Point>, lines: Vec<Line<W>>, adj_list: Vec<Vec<usize>>, dual: UndirectedGraph<W>, m: usize) -> Self {
        PlanarGraph {
            points,
            lines,
            adj_list,
            dual,
            m,
        }
    }
    pub fn f(&self) -> usize {
        self.dual.n()
    }
    pub fn points(&self, u: usize) -> &Point {
        &self.points[u]
    }
    pub fn dual(&self) -> &UndirectedGraph<W> {
        &self.dual
    }
    #[allow(non_snake_case)]
    pub fn N(&self, u: usize) -> Vec<Line<W>> {
        self.adj_list[u]
            .iter()
            .map(|v| self.lines[*v].clone())
            .collect()
    }
}

impl <'a, W: Weight> Graph<'a, Point, Line<W>> for PlanarGraph<W> {
    fn n(&self) -> usize { self.points.len() }
    fn m(&self) -> usize { self.m }
    fn vertices(&'a self) -> impl Iterator<Item = Point> {
        self.points.clone().into_iter()
    }
    fn add_edge(&mut self, _: Point, e: Line<W>) {
        let b = e.reverse();
        self.adj_list[e.from].push(self.lines.len());
        self.lines.push(e);
        self.adj_list[b.from].push(self.lines.len());
        self.lines.push(b);
        self.m += 1;
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
    use crate::structure::planar::planar_graph::PlanarGraph;

    #[test]
    fn test_small_planar1() {
        let planar: PlanarGraph<f64> = std::fs::read_to_string("data/planar_graphs/small_planar1/small_planar1.in")
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







