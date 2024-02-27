use std::fmt::{Debug, Formatter};
use std::ops::{Index, IndexMut};
use std::str::{FromStr};
use crate::structure::graph::Graph;

#[derive(PartialEq, Clone)]
pub struct UndirectedGraph {
    adj_list: Vec<Vec<usize>>,
    n: usize,
    m: usize,
}
impl UndirectedGraph {
    pub fn new(n: usize) -> Self {
        UndirectedGraph {
            adj_list: (0..n).map(|_| Vec::new()).collect(),
            n,
            m: 0,
        }
    }
}

impl Graph<usize, usize> for UndirectedGraph {
    fn n(&self) -> usize { self.n }
    fn m(&self) -> usize { self.m }
    fn vertices(&self) -> Vec<usize> { (0..self.n).collect() }
    fn neighbourhood(&self, u: &usize) -> &Vec<usize> { &self.adj_list[*u] }
    fn add_edge(&mut self, u: usize, v: usize) {
        self.adj_list[u].push(v);
        self.adj_list[v].push(u);
        self.m = self.m + 1
    }
}

impl From<String> for UndirectedGraph {
    fn from(value: String) -> Self {
        Self::from_str(value.as_str())
            .expect(format!(
                "Could not parse the following as an UndirectedGraph: \n{}",
                &value[..50.min(value.len()-1)]
            ).as_str())
    }
}

impl FromStr for UndirectedGraph {
    type Err = String;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let mut ls = str.lines()
            .map(str::trim)
            .filter(|&l| l.len() > 0 && ! l.starts_with("%"));
        let row1 = ls.next().ok_or("Expected an integer denoting the number of vertices, but found nothing!".to_owned())?;
        let n = row1.split(' ').next().unwrap().parse().or(Err(format!("Could not parse '{}' as n", row1)))?;
        let mut ret = UndirectedGraph::new(n);
        for row in ls {
            let mut rs = row.split(' ');
            let u = ret.parse_vertex(&mut rs)?;
            let v = ret.parse_vertex(&mut rs)?;
            ret.add_edge(u, v);
        }
        Ok(ret)
    }
}

impl Index<usize> for UndirectedGraph {
    type Output = Vec<usize>;

    fn index(&self, u: usize) -> &Self::Output {
        &self.adj_list[u]
    }
}

impl IndexMut<usize> for UndirectedGraph {
    fn index_mut(&mut self, u: usize) -> &mut Self::Output {
        &mut self.adj_list[u]
    }
}

impl Debug for UndirectedGraph {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut ret = String::new();
        ret.push_str(format!("UndirectedGraph(n = {}, m = {}):\n", self.n, self.m).as_str());
        for u in self.vertices() {
            ret.push_str(format!("  N({}) = {:?}\n", u, self[u]).as_str());
        }
        write!(f, "{}", ret)
    }
}