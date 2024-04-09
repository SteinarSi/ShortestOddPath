use std::collections::BTreeSet;
use crate::structure::graph::edge::{BasicEdge, Edge};
use crate::structure::graph::graph::Graph;
use crate::structure::graph::undirected_graph::UndirectedGraph;
use crate::structure::weight::{Weight};

pub fn split_edges<W, I, E>(g: &UndirectedGraph<W,E>, f: I) -> UndirectedGraph<W,BasicEdge<W>>
    where W: Weight,
          I: IntoIterator<Item = (usize,usize)>,
          E: Edge<W>,
{
    // Make sure that all the banned edges are ordered, so we can check other edges quicker
    let bans: BTreeSet<(usize,usize)> = f.into_iter().map(|(u,v)| if v < u {(v,u)} else {(u,v)} ).collect();
    let n = g.n() + g.m() - bans.len();
    let mut m = g.n();
    let mut ret = UndirectedGraph::new(n);

    for u in g.vertices() {
        for e in g[&u].iter().filter(|&e| u < e.to()) {
            if bans.contains(&(u,e.to())) {
                ret.add_edge(BasicEdge::new(u, e.to(), e.weight()));
            }
            else {
                // Split the weight into two, so that the sum of the new edges is equal to the original edge.
                // Why not divide them equally? Because we don't know if we're working with integers or floats or
                //  cyclotomics or whatever, and don't want to risk off-by-one errors.
                ret.add_edge(BasicEdge::new(u, m, e.weight()));
                ret.add_edge(BasicEdge::new(m, e.to(), 0.into()));
                m += 1;
            }
        }
    }
    ret
}
