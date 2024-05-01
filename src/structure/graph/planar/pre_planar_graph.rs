use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use crate::structure::graph::edge::{Edge, map_to};
use crate::structure::graph::graph::Graph;
use crate::structure::graph::planar::planar_edge::{PlanarEdge};
use crate::structure::graph::planar::planar_graph::PlanarGraph;
use crate::structure::graph::planar::point::{compare_edges_clockwise, Point};
use crate::structure::graph::planar::pre_planar_edge::PrePlanarEdge;
use crate::structure::graph::undirected_graph::UndirectedGraph;
use crate::structure::weight::Weight;
use crate::utility::misc::repeat;

pub struct PrePlanarGraph<W: Weight> {
    points: Vec<Option<Point>>,
    adj_list: Vec<Vec<PrePlanarEdge<W>>>,
    m: usize,
}

impl <W: Weight> PrePlanarGraph<W> {
    pub fn empty(n: usize) -> Self {
        PrePlanarGraph {
            points: repeat(n, None),
            adj_list: repeat(n, Vec::new()),
            m: 0,
        }
    }
    pub fn add_vertex(&mut self, u: Point) {
        let i = u.id;
        self.points[i] = Some(u);
        self.adj_list.push(Vec::new());
    }

    pub fn planarize(mut self) -> Result<PlanarGraph<W>, &'static str> {
        let mut points = Vec::new();
        for p in &self.points {
            points.push(p.clone().ok_or("Not all points have been defined")?);
        }
        self.sort_edges(&points);
        self.determine_faces(&points)?;

        let f = self.m() - self.n() + 2;
        let adj_list = self.adj_list
            .into_iter()
            .map(|xs| xs.into_iter().map(PrePlanarEdge::planarize).collect())
            .collect();
        let dual = Self::construct_dual(f, &adj_list);

        Ok(PlanarGraph::new(
            points,
            adj_list,
            dual,
            self.m,
        ))
    }

    fn sort_edges(&mut self, points: &Vec<Point>) {
        for u in 0..self.n() {
            self.adj_list[u].sort_by(compare_edges_clockwise(&points[u], &points));
        }
    }

    fn determine_faces(&mut self, points: &Vec<Point>) -> Result<(), &'static str> {
        let adj_list_copy = self.adj_list.clone();
        let mut current_face = 0;
        for start_vertex in 0..self.n() {
            for mut curr_line_id in 0..self.adj_list[start_vertex].len() {
                let mut curr_line = &adj_list_copy[start_vertex][curr_line_id];
                // let mut curr_line = &edges_copy[self.adj_list[start_vertex][curr_line_id]];
                // if self.edges[self.adj_list[start_vertex][curr_line_id]].left.is_none() {
                if self.adj_list[start_vertex][curr_line_id].left.is_none() {
                    loop {
                        self.adj_list[curr_line.from][curr_line_id].left = Some(current_face);
                        // self.edges[self.adj_list[curr_line.from][curr_line_id]].left = Some(current_face);
                        let id = (0..adj_list_copy[curr_line.to].len())
                            .find(|&i| adj_list_copy[curr_line.to][i].to == curr_line.from)
                            .expect("Couldn't find the reverse edge");
                        // let id = self.adj_list[curr_line.to]
                        //     .iter().position(|&l| edges_copy[l].to == curr_line.from)
                        //     .expect("Couldn't find the reverse edge");
                        self.adj_list[curr_line.to][id].right = Some(current_face);
                        // self.edges[self.adj_list[curr_line.to][id]].right = Some(current_face);
                        curr_line_id = (id + 1) % self.adj_list[curr_line.to].len();
                        curr_line = &adj_list_copy[curr_line.to][curr_line_id];
                        // curr_line = &edges_copy[self.adj_list[curr_line.to][curr_line_id]];

                        if curr_line.from == start_vertex {
                            break;
                        }
                    }
                    current_face += 1;
                }
            }
        }

        for u in points {
            for e in &self.adj_list[u.id] {
                if e.left.is_none() || e.right.is_none() {
                    return Err("Not all edges found both a left and right region!");
                }
            }
        }

        if self.n() + current_face - self.m != 2 {
            return Err("Either we don't have the correct faces, or Euler's formula is wrong :thinkin:");
        }
        Ok(())
    }

    fn construct_dual(f: usize, lines: &Vec<Vec<PlanarEdge<W>>>) -> UndirectedGraph<W,PlanarEdge<W>> {
        let mut dual = UndirectedGraph::new(f);
        for u in 0..lines.len() {
            for e in &lines[u] {
                dual.add_edge(e.rotate_right());
            }
        }
        dual
    }
}

impl <'a, W: Weight> Graph<'a, PrePlanarEdge<W>, W> for PrePlanarGraph<W> {
    type V = Option<Point>;
    fn n(&self) -> usize {
        self.points.len()
    }

    fn m(&self) -> usize {
        self.m
    }

    fn vertices(&'a self) -> impl Iterator<Item = Option<Point>> {
        self.points.clone().into_iter()
    }

    #[allow(non_snake_case)]
    fn N(&self, u: usize) -> &Vec<PrePlanarEdge<W>> { &self.adj_list[u] }

    fn add_edge(&mut self, e: PrePlanarEdge<W>) {
        let b = e.reverse();
        self.adj_list[e.from()].push(e);
        self.adj_list[b.from()].push(b);
        self.m += 1;
    }
    fn is_adjacent(&self, u: usize, v: usize) -> bool {
        let (p, q) = if self.adj_list[u].len() < self.adj_list[v].len() {
            (u, v)
        }
        else {
            (v, u)
        };
        self.adj_list[p].iter().find(|e| e.to() == q).is_some()
    }

    fn find_edges(&self, u: usize, v: usize) -> Vec<PrePlanarEdge<W>> {
        self.adj_list[u]
            .clone()
            .into_iter()
            .filter(|e| e.to() == v)
            .collect()
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
        let mut graph = PrePlanarGraph::empty(n);
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
            if let Some(p) = u {
                ret.push_str(format!("  N({}) = {:?}", p.id, map_to(&self.adj_list[p.id])).as_str());
            }
            else {
                ret.push_str("[Point not defined yet]")
            }
        }
        write!(f, "{}", ret)
    }
}


