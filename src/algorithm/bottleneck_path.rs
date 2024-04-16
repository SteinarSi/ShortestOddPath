use crate::algorithm::odd_path::shortest_odd_path;
use crate::algorithm::utility::split_edges;
use crate::structure::graph::edge::{Edge};
use crate::structure::path_result::{PathResult, PathResult::*};
use crate::structure::graph::undirected_graph::UndirectedGraph;
use crate::structure::weight::{Weight};

/**
Problem: Shortest Bottleneck Path
In: an undirected graph G, two vertices s,t in V(G), and a 'bottleneck' edge (u,v) in E(G)
Out: the shortest s-t-path in G that passes through (u,v), if one exists
*/

pub fn shortest_bottleneck_path<W: Weight, E: Edge<W>>(graph: &UndirectedGraph<W,E>, s: usize, t: usize, bottleneck: (usize,usize)) -> PathResult<W,E> {
    let (split, map) = split_edges(&graph, vec![bottleneck]);
    match shortest_odd_path(&split, s, t) {
        Impossible => Impossible,
        Possible {cost: _, path} => {
            let ret: Vec<E> = path.iter().flat_map(|e|map(e)).collect();

            // TODO trenger jo egentlig ikke denne løkken, kan bruke cost fra odd_path
            let mut cost = 0.into();
            for e in &ret {
                cost = cost + e.weight();
            }
            Possible {
                cost,
                path: ret,
            }
        },
    }
}
