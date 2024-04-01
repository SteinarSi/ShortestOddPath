use std::collections::BTreeSet;
use crate::structure::graph::Graph;
use crate::structure::undirected_graph::UndirectedGraph;
use crate::structure::weight::Weight;

pub fn split_edges<'a, W: Weight, I>(g: &UndirectedGraph<W>, f: I) -> UndirectedGraph<W>
    where W: Weight,
          I: IntoIterator<Item = &'a (usize,usize)>,
{
    // Make sure that all the banned edges are ordered, so we can check other edges quicker
    let bans: BTreeSet<(usize,usize)> = f.into_iter().map(|(u,v)| if *v < *u {(*v,*u)} else {(*u,*v)} ).collect();
    let n = g.n() + g.m() - bans.len();
    let mut m = g.n();
    let mut ret = UndirectedGraph::new(n);

    for u in g.vertices() {
        for &(w, v) in g[&u].iter().filter(|&&(_,v)| u < v) {
            if bans.contains(&(u,v)) {
                ret.add_edge(u, (w,v));
            }
            else {
                // Split the weight into two, so that the sum of the new edges is equal to the original edge.
                // Why not divide them equally? Because we don't know if we're working with integers or floats or
                //  cyclotomics or whatever, and don't want to risk off-by-one errors.
                ret.add_edge(u, (w, m));
                ret.add_edge(m, (0.into(), v));
                m += 1;
            }
        }
    }
    ret
}
