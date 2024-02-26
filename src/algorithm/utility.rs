use std::collections::BTreeSet;
use crate::structure::graph::Graph;
use crate::structure::undirected_graph::UndirectedGraph;

pub fn split_edges<'a, I>(g: &UndirectedGraph, f: I) -> UndirectedGraph
    where I: IntoIterator<Item = &'a (usize,usize)>,
{
    // Make sure that all the banned edges are ordered, so we can check other edges quicker
    let bans: BTreeSet<(usize,usize)> = f.into_iter().map(|(u,v)| if *v < *u {(*v,*u)} else {(*u,*v)} ).collect();
    let n = g.n() + g.m() - bans.len();
    let mut m = g.n();
    let mut ret = UndirectedGraph::new(n);

    for u in g.vertices() {
        for &v in g.N(&u).iter().filter(|&&v| u < v) {
            if bans.contains(&(u,v)) {
                ret.add_edge(u, v);
            }
            else {
                ret.add_edge(u, m);
                ret.add_edge(m, v);
                m += 1;
            }
        }
    }
    ret
}
