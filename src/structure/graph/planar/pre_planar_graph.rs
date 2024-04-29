use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use crate::structure::graph::edge::{Edge};
use crate::structure::graph::graph::Graph;
use crate::structure::graph::planar::planar_edge::{PlanarEdge, PlanarEdgeImpl};
use crate::structure::graph::planar::planar_graph::PlanarGraph;
use crate::structure::graph::planar::point::{compare_edges_clockwise, Point};
use crate::structure::graph::planar::pre_planar_edge::PrePlanarEdge;
use crate::structure::graph::undirected_graph::UndirectedGraph;
use crate::structure::weight::Weight;

pub struct PrePlanarGraph<W: Weight> {
    points: Vec<Point>,
    edges: Vec<PrePlanarEdge<W>>,
    adj_list: Vec<Vec<usize>>,
    m: usize,
}

impl <W: Weight> PrePlanarGraph<W> {
    pub fn empty() -> Self {
        PrePlanarGraph {
            points: Vec::new(),
            edges: Vec::new(),
            adj_list: Vec::new(),
            m: 0,
        }
    }
    pub fn add_vertex(&mut self, u: Point) {
        self.points.push(u);
        self.adj_list.push(Vec::new());
    }

    pub fn planarize(mut self) -> Result<PlanarGraph<W>, &'static str> {
        self.sort_edges();
        self.determine_faces()?;

        let f = self.m() - self.n() + 2;
        let lines = self.edges.into_iter()
            .map(|l| l.planarize())
            .collect();
        let dual = Self::construct_dual(f, &lines);

        Ok(PlanarGraph::new(
            self.points,
            lines,
            self.adj_list,
            dual,
        ))
    }

    fn sort_edges(&mut self) {
        for u in 0..self.n() {
            self.adj_list[u].sort_by(compare_edges_clockwise(&self.points[u], &self.points, &self.edges));
        }
    }

    fn determine_faces(&mut self) -> Result<(), &'static str> {
        let edges_copy = self.edges.clone();
        let mut current_face = 0;
        for start_vertex in 0..self.n() {
            for mut curr_line_id in 0..self.adj_list[start_vertex].len() {
                let mut curr_line = &edges_copy[self.adj_list[start_vertex][curr_line_id]];
                if self.edges[self.adj_list[start_vertex][curr_line_id]].left.is_none() {
                    loop {
                        self.edges[self.adj_list[curr_line.from][curr_line_id]].left = Some(current_face);
                        let id = self.adj_list[curr_line.to]
                            .iter().position(|&l| edges_copy[l].to == curr_line.from)
                            .expect("Couldn't find the reverse edge");
                        self.edges[self.adj_list[curr_line.to][id]].right = Some(current_face);
                        curr_line_id = (id + 1) % self.adj_list[curr_line.to].len();
                        curr_line = &edges_copy[self.adj_list[curr_line.to][curr_line_id]];

                        if curr_line.from == start_vertex {
                            break;
                        }
                    }
                    current_face += 1;
                }
            }
        }

        for u in self.vertices() {
            for &v in &self.adj_list[u.id] {
                let edge = &self.edges[v];
                if edge.left.is_none() || edge.right.is_none() {
                    return Err("Not all edges found both a left and right region!");
                }
            }
        }

        if self.n() + current_face - self.m != 2 {
            return Err("Either we don't have the correct faces, or Euler's formula is wrong :thinkin:");
        }
        Ok(())
    }

    fn construct_dual(f: usize, lines: &Vec<PlanarEdgeImpl<W>>) -> UndirectedGraph<W,PlanarEdgeImpl<W>> {
        let mut dual = UndirectedGraph::new(f);
        for i in (0..lines.len()).step_by(2) {
            let e = &lines[i];
            dual.add_edge(e.rotate_right());
        }
        dual
    }
}

impl <'a, W: Weight> Graph<'a, PrePlanarEdge<W>, W> for PrePlanarGraph<W> {
    type V = Point;
    fn n(&self) -> usize {
        self.points.len()
    }

    fn m(&self) -> usize {
        self.m
    }

    fn vertices(&'a self) -> impl Iterator<Item = Point> {
        self.points.clone().into_iter()
    }

    fn N(&self, u: usize) -> Vec<PrePlanarEdge<W>> {
        self.adj_list[u].iter().map(|i| self.edges[*i].clone()).collect()
    }

    fn add_edge(&mut self, e: PrePlanarEdge<W>) {
        let b = e.reverse();
        self.adj_list[e.from].push(self.edges.len());
        self.edges.push(e);
        self.adj_list[b.from].push(self.edges.len());
        self.edges.push(b);
        self.m += 1;
    }
    fn is_adjacent(&self, u: usize, v: usize) -> bool {
        self.adj_list[u].iter().find(|e|self.edges[**e].to == v).is_some()
    }

    fn find_edges(&self, u: usize, v: usize) -> Vec<PrePlanarEdge<W>> {
        self.adj_list[u].iter().map(|i| self.edges[*i].clone()).filter(|e| e.to() == v).collect()
    }
}

impl <W: Weight> From<String> for PrePlanarGraph<W> {
    fn from(value: String) -> Self {
        PrePlanarGraph::from_str(value.as_str())
            .expect(format!(
                "Could not parse the following as an PlanarGraph: \n{}",
                &value[..50.min(value.len()-1)]
            ).as_str())
    }
}

impl <W: Weight> FromStr for PrePlanarGraph<W> {
    type Err = &'static str;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
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
        let mut graph = PrePlanarGraph::empty();
        for _ in 0..n {
            graph.add_vertex(
                ls.next().ok_or("Expected another vertex here, but got nothing")?
                .parse()?
            );
        }
        for _ in 0..m {
            graph.add_edge(
                ls.next().ok_or("Expected another edge here, but got nothing")?
                .parse()?
            );
        }
        Ok(graph)
    }
}

impl <W: Weight> Debug for PrePlanarGraph<W> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut ret = String::new();
        ret.push_str(format!("PlanarGraph(n = {}, m = {}):\n", self.n(), self.m()).as_str());
        for u in self.vertices() {
            ret.push_str(format!("  N({}) = {:?}\n", u.id, self.adj_list[u.id].iter().map(|p| {
                let e = &self.edges[*p];
                if e.from == u.id { e.to } else { e.from }
            }).collect::<Vec<usize>>()).as_str())
        }
        write!(f, "{}", ret)
    }
}


