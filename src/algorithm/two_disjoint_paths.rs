use crate::structure::path_result::PathResult::*;
use crate::algorithm::bottleneck_path::shortest_bottleneck_path;
use crate::structure::edge::BasicEdge;
use crate::structure::undirected_graph::UndirectedGraph;
use crate::structure::weight::Weight;

/**
Problem: 2-Disjoint Paths
In: an undirected graph G, and four vertices s1,s2,t1,t2 in V(G)
Out: an s1-t1-path and an s2-t2-path in G, whose vertices are distinct, minimizing the sum of their lengths.
*/

pub fn two_disjoint_paths<W: Weight>(graph: &UndirectedGraph<W,BasicEdge<W>>, (s1, t1): (usize, usize), (s2, t2): (usize, usize)) -> Option<(W, Vec<usize>, Vec<usize>)> {
    let mut g = graph.clone();
    g.remove_edge(&(t1,t2));
    unsafe {
        g.add_directed_edge(BasicEdge::new(t1, t2, 0.into()));
    }
    match shortest_bottleneck_path(&g, s1, s2, (t1,t2)) {
        Impossible => None,
        Possible {cost: w, mut path} => {
            let i = path.iter().position(|&u| u == t1).expect("The bottleneck path didn't go through the bottleneck of t1,t2");
            let p1 = path[0..=i].to_vec();
            let p2 = &mut path[i+1..];
            p2.reverse();
            Some((w, p1, p2.to_vec()))
        }
    }
}
