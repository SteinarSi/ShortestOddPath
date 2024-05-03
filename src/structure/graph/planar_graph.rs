use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use crate::structure::graph::edge::Edge;
use crate::structure::graph::planar_edge::{PlanarEdge, PrePlanarEdge};
use crate::structure::graph::point::{compare_edges_clockwise, Point};
use crate::structure::graph::simple_graph_strategy::{SimpleGraphStrategy, SumWeights};
use crate::structure::graph::undirected_graph::UndirectedGraph;
use crate::structure::weight::Weight;
use crate::utility::misc::repeat;

pub struct PlanarGraph<W: Weight> {
    real: UndirectedGraph<W, PlanarEdge<W>>,
    dual: UndirectedGraph<W, PlanarEdge<W>>,
}

impl <W: Weight> PlanarGraph<W> {
    pub fn real(&self) -> &UndirectedGraph<W, PlanarEdge<W>> { &self.real }
    pub fn dual(&self) -> &UndirectedGraph<W, PlanarEdge<W>> { &self.dual }
    pub fn n(&self) -> usize { self.real.n() }
    pub fn m(&self) -> usize { self.real.m() }
    pub fn f(&self) -> usize { self.dual.n() }
    pub fn parse<S: SimpleGraphStrategy>(str: &str) -> Result<Self, &'static str> {
        let mut ls = str.lines()
            .map(str::trim)
            .filter(|&l| l.len() > 0 && ! l.starts_with("%"));
        let mut row1 = ls.next()
            .ok_or("Could not find the first row")?
            .split(' ')
            .map(usize::from_str);
        let n = row1.next()
            .ok_or("Could not find n")?
            .or(Err("Could not parse n"))?;
        let m = row1.next()
            .ok_or("Could not find m")?
            .or(Err("Could not parse m"))?;
        let mut pre = PrePlanarGraph::empty(n);

        for _ in 0..n {
            pre.add_vertex(ls.next().ok_or("Expected another vertex here, but got nothing")?.parse()?);
        }
        for _ in 0..m {
            pre.add_edge::<S>(ls.next().ok_or("Expected another edge here, but got nothing")?.parse()?);
        }
        Ok(pre.planarize()?)
    }
}

struct PrePlanarGraph<W: Weight> {
    graph: UndirectedGraph<W, PrePlanarEdge<W>>,
    points: Vec<Option<Point>>,
}

impl <W: Weight> PrePlanarGraph<W> {
    pub fn empty(n: usize) -> Self {
        PrePlanarGraph {
            graph: UndirectedGraph::new(n),
            points: repeat(n, None),
        }
    }
    pub fn add_vertex(&mut self, u: Point) {
        let i = u.id;
        self.points[i] = Some(u);
    }

    pub fn add_edge<S: SimpleGraphStrategy>(&mut self, x: PrePlanarEdge<W>) {
        let (u,v, e) = if self.graph.adj_list[x.from].len() < self.graph.adj_list[x.to].len() {
            (x.from(), x.to(), x)
        } else {
            (x.to(), x.from(), x.reverse())
        };
        if let Some(i) = self.graph.adj_list[u].iter().position(|x| x.to == v) {
            let b = e.reverse();
            self.graph.adj_list[u][i] = S::combine(e, self.graph.adj_list[u][i].clone());
            let j = self.graph.adj_list[v].iter()
                .position(|v| v.to == u)
                .expect("Uhm, looks like we have a uni-directional edge here");
            self.graph.adj_list[v][j] = S::combine(b, self.graph.adj_list[v][j].clone());
        }
        else {
            self.graph.add_edge(e);
        }
    }

    pub fn planarize(mut self) -> Result<PlanarGraph<W>, &'static str> {
        let mut points = Vec::new();
        for p in &self.points {
            points.push(p.clone().ok_or("Not all points have been defined")?);
        }
        self.sort_edges(&points);
        let f = self.determine_faces(&points)?;

        let mut real = UndirectedGraph::new(self.graph.n());
        let mut dual = UndirectedGraph::new(f);
        self.graph.adj_list.iter().for_each(|xs| {
            xs.iter()
                .filter(|e| e.from() < e.to())
                .for_each(|e| {
                let p = e.planarize();
                let b = p.rotate_right();
                real.add_edge(p);
                dual.add_edge(b);
            })
        });

        Ok(PlanarGraph {
            real,
            dual,
        })
    }
    fn sort_edges(&mut self, points: &Vec<Point>) {
        for u in 0..self.graph.n() {
            self.graph
                .adj_list[u]
                .sort_by(compare_edges_clockwise(&points[u], &points));
        }
    }
    fn determine_faces(&mut self, points: &Vec<Point>) -> Result<usize, &'static str> {
        let n = self.graph.n();
        let adj_list = &mut self.graph.adj_list;
        let adj_list_copy = adj_list.clone();
        let mut current_face = 0;
        for start_vertex in 0..n {
            for mut curr_line_id in 0..adj_list[start_vertex].len() {
                let mut curr_line = &adj_list_copy[start_vertex][curr_line_id];
                if adj_list[start_vertex][curr_line_id].left.is_none() {
                    loop {
                        adj_list[curr_line.from][curr_line_id].left = Some(current_face);
                        let id = adj_list_copy[curr_line.to]
                            .iter()
                            .position(|e| e.to == curr_line.from)
                            .expect("Couldn't find the reverse edge");
                        adj_list[curr_line.to][id].right = Some(current_face);
                        curr_line_id = (id + 1) % adj_list[curr_line.to].len();
                        curr_line = &adj_list_copy[curr_line.to][curr_line_id];

                        if curr_line.from == start_vertex {
                            break;
                        }
                    }
                    current_face += 1;
                }
            }
        }

        for u in points {
            for e in &adj_list[u.id] {
                if e.left.is_none() || e.right.is_none() {
                    return Err("Not all edges found both a left and right region!");
                }
            }
        }
        if self.graph.m() > n + current_face || n + current_face - self.graph.m() != 2 {
            println!("n = {}, m = {}, f = {}", self.graph.n(), self.graph.m(), current_face);
            println!("Either we don't have the correct faces, or Euler's formula is wrong :thinkin:");
            // return Err("Either we don't have the correct faces, or Euler's formula is wrong :thinkin:");
        }
        Ok(current_face)
    }
}

impl <W: Weight> Debug for PlanarGraph<W> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "PlanarGraph(n = {}, m = {}, f = {}):\n", self.n(), self.m(), self.f())?;
        write!(f, "Real part:\n")?;
        self.real.fmt(f)?;
        write!(f, "Dual part:\n")?;
        self.dual.fmt(f)?;
        Ok(())
    }
}

impl <W: Weight> FromStr for PlanarGraph<W> {
    type Err = &'static str;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        Self::parse::<SumWeights>(str)
    }
}

mod test_planar_graph {
    use crate::structure::graph::planar_graph::PlanarGraph;

    fn parse(name: &str) -> PlanarGraph<f64> {
        std::fs::read_to_string(["data/planar_graphs/small_planar_graphs/", name, "/", name, ".in"].concat())
            .expect("No graph found")
            .parse::<PlanarGraph<f64>>()
            .expect("Could not parse graph")
    }

    #[test]
    fn test_small_planar1() {
        let planar = parse("small_planar1");

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

    #[test]
    fn test_small_planar2() {
        let planar = parse("small_planar2");

        assert!(planar.dual().is_adjacent(0, 0));
        assert!(planar.dual().is_adjacent(0, 1));
        assert!(planar.dual().is_adjacent(1, 2));
        assert!(planar.dual().is_adjacent(1, 6));
        assert!(planar.dual().is_adjacent(5, 6));
        assert!(planar.dual().is_adjacent(4, 5));
        assert!(planar.dual().is_adjacent(4, 7));
        assert!(planar.dual().is_adjacent(4, 2));
        assert!(planar.dual().is_adjacent(7, 8));
        assert!(planar.dual().is_adjacent(8, 0));

        assert!( ! planar.dual().is_adjacent(1, 7));
        assert!( ! planar.dual().is_adjacent(2, 5));
        assert!( ! planar.dual().is_adjacent(5, 3));
        assert!( ! planar.dual().is_adjacent(5, 0));
        assert!( ! planar.dual().is_adjacent(5, 5));

        println!("{:?}", planar);
    }

    #[test]
    fn test_small_planar3() {
        let planar = parse("small_planar3");

        println!("{:?}", planar);
    }
}