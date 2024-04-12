use std::collections::BTreeSet;
use crate::structure::graph::edge::{BasicEdge, Edge};
use crate::structure::graph::graph::Graph;
use crate::structure::graph::undirected_graph::UndirectedGraph;
use crate::structure::weight::{Weight};

//                                                                                    TODO burde være en vilkårlige Edge<W> istedet for BasicEdge eller E
pub fn split_edges<W, I, E>(g: &UndirectedGraph<W,E>, f: I) -> (UndirectedGraph<W,E>, impl Fn(&BasicEdge<W>) -> Option<BasicEdge<W>>)
    where W: Weight,
          I: IntoIterator<Item = (usize,usize)>,
          E: Edge<W>,
{
    // Make sure that all the banned edges are ordered, so we can check other edges quicker
    let bans: BTreeSet<(usize,usize)> = f.into_iter().map(|(u,v)| if v < u {(v,u)} else {(u,v)} ).collect();
    let extra = g.m() - bans.len();
    let old_n = g.n();
    let new_n = g.n() + extra;
    let mut m = g.n();
    let mut map = Vec::new();
    let mut split = UndirectedGraph::new(new_n);

    for u in g.vertices() {
        for e in g[&u].iter().filter(|&e| u < e.to()) {
            if bans.contains(&(u,e.to())) {
                split.add_edge(e.clone());
            }
            else {
                let (a, b) = e.subdivide(m);
                split.add_edge(a);
                split.add_edge(b);
                map.push(e.clone());
                m += 1;
            }
        }
    }

    (split, move |e| {
        if e.from() >= old_n {
            None
        }
        else if e.to() < old_n {
            Some(e.clone())
        }
        else {
            let b = &map[e.to() - old_n];
            if b.from() == e.from() {
                // TODO dette burde vært generisk
                Some(BasicEdge::new(b.from(), b.to(), b.weight()))
                // Some(b.clone())
            }
            else {
                // TODO burde være generisk
                let bb = b.reverse();
                Some(BasicEdge::new(bb.from(), bb.to(), bb.weight()))
                // Some(b.reverse())
            }

        }
    })
}

pub fn create_mirror_graph<W: Weight,E: Edge<W>>(graph: &UndirectedGraph<W,E>, s: usize, t: usize) -> UndirectedGraph<W,BasicEdge<W>> {
    let orig_n = graph.n();
    let new_n = orig_n * 2;
    let mut mirror = UndirectedGraph::new(new_n);
    for u in graph.vertices() {
        mirror[&u] = graph[&u].iter()
            .map(|e| BasicEdge::new(e.from(), e.to(), e.weight()))
            .collect();
        if u != s && u != t {
            mirror[&(u + orig_n)] = graph[&u].iter()
                .filter(|e| e.to() != s && e.to() != t)
                .map(|e| BasicEdge::new(u + orig_n, e.to() + orig_n, e.weight()))
                .collect()
        }
    }
    mirror
}
