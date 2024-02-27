use std::collections::BTreeSet;
use crate::algorithm::odd_path::shortest_odd_path;
use crate::algorithm::utility::split_edges;
use crate::structure::graph::Graph;
use crate::structure::path_result::{PathResult, PathResult::*};
use crate::structure::undirected_graph::UndirectedGraph;

/**
Problem: Shortest Bottleneck Path
In: an undirected graph G, two vertices s,t in V(G), and a 'bottleneck' edge (u,v) in E(G)
Out: the shortest s-t-path in G that passes through (u,v), if one exists
*/

pub fn shortest_bottleneck_path(graph: &UndirectedGraph, s: usize, t: usize, bottleneck: (usize,usize)) -> PathResult {
    let split = split_edges(&graph, &BTreeSet::from([bottleneck]));
    match shortest_odd_path(split, s, t) {
        Impossible => Impossible,
        Possible {cost: _, path} => {
            let p: Vec<usize> = path.into_iter().filter(|&u| u < graph.n()).collect();
            let c = p.len() - 1;
            Possible {
                cost: c as u64,
                path: p,
            }
        },
    }
}
