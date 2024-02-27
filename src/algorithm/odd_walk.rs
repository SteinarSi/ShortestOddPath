use crate::structure::{
    undirected_graph::UndirectedGraph,
    graph::Graph,
    cost::{Cost, Cost::*},
    path_result::{PathResult, PathResult::*},
};
use std::collections::VecDeque;
use crate::utility::misc::repeat;

/**
Problem: Shortest Odd Walk
In: an undirected graph G, and two vertices s,t in V(G)
Out: the shortest s-t-walk in G, that uses an odd number of edges
*/

pub fn shortest_odd_walk(graph: UndirectedGraph, s: usize, t: usize) -> PathResult {
    let n = graph.n();
    let mut even_dist: Vec<Cost> = repeat(n, Infinite);
    even_dist[s] = Finite(0);
    let mut odd_dist = repeat(n, Infinite);
    let mut queue: VecDeque<(usize, u64)> = VecDeque::from([(s, 0)]);
    let mut even_prev = repeat(n, None);
    let mut odd_prev = repeat(n, None);

    while ! queue.is_empty() {
        let (u, distu) = queue.pop_front().unwrap();

        for &v in graph.neighbourhood(&u) {
            let distv = distu + 1;
            if distv % 2 == 0 && Finite(distv) < even_dist[v] {
                even_dist[v] = Finite(distv);
                queue.push_back((v, distv));
                even_prev[v] = Some(u);
            }
            if distv % 2 == 1 && Finite(distv) < odd_dist[v] {
                odd_dist[v] = Finite(distv);
                queue.push_back((v, distv));
                odd_prev[v] = Some(u);
            }
        }
        if odd_dist[t].is_finite() { break; }
    }

    match odd_dist[t] {
        Infinite => Impossible,
        Finite(cost) => {
            let mut v = odd_prev[t].unwrap();
            let mut path = vec![t, v];
            while v != s {
                v = even_prev[v].unwrap();
                path.push(v);
                v = odd_prev[v].unwrap();
                path.push(v);
            }
            path.reverse();
            Possible {
                cost,
                path,
            }
        }
    }
}
