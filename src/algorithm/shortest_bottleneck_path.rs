use crate::algorithm::shortest_odd_path::shortest_odd_path;
use crate::algorithm::utility::split_edges;
use crate::structure::graph::edge::{Edge};
use crate::structure::path_result::{PathResult, PathResult::*};
use crate::structure::graph::undirected_graph::UndirectedGraph;
use crate::structure::weight::{Weight};

/**
Problem: Shortest Bottleneck Path
In: an undirected graph G, two vertices s,t in V(G), and a 'bottleneck' edge (u,v) in E(G)
Out: an s-t-path in G of minimum cost that passes through (u,v), if one exists
*/

pub fn shortest_bottleneck_path<W: Weight, E: Edge<W>>(graph: &UndirectedGraph<W,E>, s: usize, t: usize, (bottle_from, bottle_to): (usize,usize)) -> PathResult<W,E> {
    let bottleneck = graph[&bottle_from].iter().filter(|e| e.to() == bottle_to).map(|e| e.clone()).collect();
    let (split, map) = split_edges(&graph, bottleneck);
    match shortest_odd_path(&split, s, t) {
        Impossible => Impossible,
        Possible {cost, path} => {
            Possible {
                cost,
                path: path
                    .iter()
                    .flat_map(|e|map(e))
                    .collect(),
            }
        },
    }
}
