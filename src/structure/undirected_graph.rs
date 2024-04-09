use std::fmt::{Debug, Formatter};
use std::ops::{Index, IndexMut};
use std::str::FromStr;
use crate::structure::graph::Graph;
use crate::structure::weight::Weight;

#[derive(PartialEq, Clone)]
pub struct UndirectedGraph<W: Weight> {
    adj_list: Vec<Vec<(W,usize)>>,
    n: usize,
    m: usize,
}
impl <W: Weight> UndirectedGraph<W> {
    pub fn new(n: usize) -> Self {
        UndirectedGraph {
            adj_list: (0..n).map(|_| Vec::new()).collect(),
            n,
            m: 0,
        }
    }

    pub fn remove_edge(&mut self, (u,v): &(usize,usize)) {
        let len = self.adj_list[*u].len();
        self.adj_list[*u].retain(|(_,w)| w != v);
        self.adj_list[*v].retain(|(_,w)| w != u);
        if len != self.adj_list[*u].len() {
            self.m = self.m - 1;
        }
    }
    pub(crate) unsafe fn add_directed_edge(&mut self, u: usize, e: (W, usize)) {
        self.adj_list[u].push(e);
        self.m = self.m + 1;
    }

    pub fn is_adjacent(&self, u: usize, v: usize) -> bool {
        self.adj_list[u].iter().find(|(_,w)| w == &v).is_some()
    }
}

impl <'a, W: Weight> Graph<'a, usize, (W, usize)> for UndirectedGraph<W> {
    fn n(&self) -> usize { self.n }
    fn m(&self) -> usize { self.m }
    fn vertices(&'a self) -> impl Iterator<Item = usize> { 0..self.n }
    fn add_edge(&mut self, u: usize, (w, v): (W, usize)) {
        self.adj_list[u].push((w, v));
        self.adj_list[v].push((w, u));
        self.m = self.m + 1
    }
}

impl <W: Weight> From<String> for UndirectedGraph<W> {
    fn from(value: String) -> Self {
        Self::from_str(value.as_str())
            .expect(format!(
                "Could not parse the following as an UndirectedGraph: \n{}",
                &value[..50.min(value.len()-1)]
            ).as_str())
    }
}

impl <W: Weight> FromStr for UndirectedGraph<W> {
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
            let www = rs.next().unwrap_or_else(|| "1");
            let w = W::from_str(www).unwrap_or_else(|_|1.into());
            ret.add_edge(u, (w, v));
        }
        Ok(ret)
    }
}

impl <W: Weight> Index<&usize> for UndirectedGraph<W> {
    type Output = Vec<(W, usize)>;

    fn index(&self, u: &usize) -> &Self::Output {
        &self.adj_list[*u]
    }
}

impl <W: Weight> IndexMut<&usize> for UndirectedGraph<W> {
    fn index_mut(&mut self, u: &usize) -> &mut Self::Output {
        &mut self.adj_list[*u]
    }
}

impl <W: Weight> Debug for UndirectedGraph<W> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut ret = String::new();
        ret.push_str(format!("UndirectedGraph(n = {}, m = {}):\n", self.n, self.m).as_str());
        for u in self.vertices() {
            ret.push_str(format!("  N({}) = {:?}\n", u, self[&u]).as_str());
        }
        write!(f, "{}", ret)
    }
}