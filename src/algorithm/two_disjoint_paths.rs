use crate::structure::path_result::PathResult::*;
use crate::algorithm::bottleneck_path::shortest_bottleneck_path;
use crate::structure::undirected_graph::UndirectedGraph;

/**
Problem: 2-Distinct Paths
In: an undirected graph G, and four vertices s1,s2,t1,t2 in V(G)
Out: an s1-t1-path and an s2-t2-path in G, whose vertices are distinct, minimizing the sum of their lengths.
*/

pub fn two_disjoint_paths(graph: &UndirectedGraph, s1: usize, s2: usize, t1: usize, t2: usize) -> Option<(Vec<usize>, Vec<usize>)> {
    match shortest_bottleneck_path(graph, s1, s2, (t1,t2)) {
        Impossible => None,
        Possible {cost: _, path} => {
            let i = path.iter().position(|&u| u == t1).unwrap();
            Some((path[0..i].to_vec(), path[i+1..].to_vec()))
        }
    }
}
