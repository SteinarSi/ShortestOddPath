use crate::structure::{
    graph::undirected_graph::UndirectedGraph,
    graph::graph::Graph,
    cost::{Cost, Cost::*},
    path_result::{PathResult, PathResult::*},
};
use std::collections::VecDeque;
use crate::structure::graph::edge::Edge;
use crate::structure::weight::Weight;
use crate::utility::misc::repeat;

/**
Problem: Shortest Odd Walk
In: an undirected graph G, and two vertices s,t in V(G)
Out: the shortest s-t-walk in G, that uses an odd number of edges
*/

pub fn shortest_odd_walk<W: Weight, E: Edge<W>>(graph: &UndirectedGraph<W,E>, s: usize, t: usize) -> PathResult<W> {
    let n = graph.n();
    let mut even_dist: Vec<Cost<W>> = repeat(n, Infinite);
    let mut odd_dist = repeat(n, Infinite);
    even_dist[s] = Finite(0.into());
    let mut queue: VecDeque<(usize, bool)> = VecDeque::from([(s, true)]);
    let mut even_prev = repeat(n, None);
    let mut odd_prev = repeat(n, None);


    while ! queue.is_empty() {
        let (u, even) = queue.pop_front().unwrap();
        if even {
            let distu = even_dist[u];
            for e in &graph[&u] {
                let distv = distu + Finite(e.weight());
                if distv < odd_dist[e.to()] {
                    odd_dist[e.to()] = distv;
                    queue.push_back((e.to(), false));
                    odd_prev[e.to()] = Some(u);
                }
            }
        }
        else {
            let distu = odd_dist[u];
            for e in &graph[&u] {
                let distv = distu + Finite(e.weight());
                if distv < even_dist[e.to()] {
                    even_dist[e.to()] = distv;
                    queue.push_back((e.to(), true));
                    even_prev[e.to()] = Some(u);
                }
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
