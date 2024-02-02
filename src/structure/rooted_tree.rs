use std::collections::BTreeSet;
use crate::structure::graph::Graph;

pub struct RootedTree {
    adj_list: Vec<Vec<usize>>,
    n: usize,
    root: usize,
    vertices: BTreeSet<usize>,
}

impl RootedTree {
    pub fn new(root: usize, n: usize) -> Self where Self: Sized {
        RootedTree {
            adj_list: (0..n).map(|_| Vec::new()).collect(),
            n,
            root,
            vertices: BTreeSet::from([root]),
        }
    }
}

impl Graph<usize, usize> for RootedTree {
    fn n(&self) -> usize {
        self.n
    }

    fn vertices(&self) -> Vec<usize> {
        self.vertices.clone().into_iter().collect()
    }

    fn neighbourhood(&self, u: &usize) -> &Vec<usize> {
        &self.adj_list[*u]
    }

    fn set_neighbourhood(&mut self, u: usize, neigh: Vec<usize>) {
        self.adj_list[u] = neigh
    }

    fn add_edge(&mut self, u: usize, e: usize) {
        self.adj_list[u].push(e);
    }

    fn add_edge_from_str(&mut self, edge: &str) -> Option<()> {
        let mut uv = edge.split(' ').map(|x| x.parse().ok());
        self.add_edge(uv.next()??, uv.next()??);
        Some(())
    }
}

impl From<String> for RootedTree {
    fn from(value: String) -> Self {
        let err = &format!("Could not parse the following as a RootedTree: \n{}", value);
        let mut ls = value.lines().map(|l| l.split(' '));
        let mut header = ls.next().expect(err);
        let n = header.next().expect(err).parse().expect(err);
        let r = header.next().expect(err).parse().expect(err);
        let mut ret = RootedTree::new(r, n);
        for mut line in ls {
            let u = line.next().expect(err).parse().expect(err);
            let v = line.next().expect(err).parse().expect(err);
            ret.add_edge(u, v);
        }
        ret
    }
}