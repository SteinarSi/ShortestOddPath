use std::fmt::{Debug, Formatter};
use std::ops::Index;
use crate::structure::graph::Graph;

#[derive(PartialEq, Clone)]
pub struct UndirectedGraph {
    adj_list: Vec<Vec<usize>>,
    n: usize,
}
impl UndirectedGraph {
    pub fn new(n: usize) -> Self {
        UndirectedGraph {
            adj_list: (0..n).map(|_| Vec::new()).collect(),
            n,
        }
    }
}

impl Graph<usize, usize> for UndirectedGraph {
    fn n(&self) -> usize {
        self.n
    }

    fn vertices(&self) -> Vec<usize> {
        (0..self.n).collect()
    }
    fn neighbourhood(&self, u: &usize) -> &Vec<usize> {
        &self.adj_list[*u]
    }
    fn set_neighbourhood(&mut self, u: usize, neigh: Vec<usize>) {
        self.adj_list[u] = neigh;
    }

    fn add_edge(&mut self, u: usize, v: usize) {
        self.adj_list[u].push(v);
        self.adj_list[v].push(u);
    }

    fn add_edge_from_str(&mut self, edge: &str) -> Option<()> {
        let mut uv = edge.split(' ').map(|x| x.parse().ok());
        let u = uv.next()??;
        let v = uv.next()??;
        self.add_edge(u, v);
        Some(())
    }
}

impl From<String> for UndirectedGraph {
    fn from(value: String) -> Self {
        let err = &format!("Could not parse the following as an UndirectedGraph: \n{}", value);
        let mut ls = value.lines().filter(|&l| ! l.trim_start().starts_with("%"));
        let n = ls.next().expect("No n :(").split(' ').next().expect(err).parse().expect(err);
        let mut ret = UndirectedGraph::new(n);
        for mut line in ls.map(|l| l.split(' ')) {
            let u = line.next().expect(err).parse().expect(err);
            let v = line.next().expect(err).parse().expect(err);
            ret.add_edge(u, v);
        }
        ret
    }
}

impl Index<usize> for UndirectedGraph {
    type Output = Vec<usize>;

    fn index(&self, u: usize) -> &Self::Output {
        &self.adj_list[u]
    }
}

impl Debug for UndirectedGraph {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut ret = String::new();
        ret.push_str(format!("UndirectedGraph(n = {}):\n", self.n).as_str());
        for u in self.vertices() {
            ret.push_str(format!("  N({}) = {:?}\n", u, self[u]).as_str());
        }
        write!(f, "{}", ret)
    }
}