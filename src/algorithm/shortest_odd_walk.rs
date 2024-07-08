use std::cmp::Reverse;
use crate::structure::{
    graph::undirected_graph::UndirectedGraph,
    cost::{Cost, Cost::*},
    path_result::{PathResult, PathResult::*},
};
use std::collections::BinaryHeap;
use crate::structure::graph::edge::Edge;
use crate::structure::weight::{Order, Weight};
use crate::utility::misc::repeat;

/**
Problem: Shortest Odd Walk
In: an undirected graph G, and two vertices s,t in V(G)
Out: the shortest s-t-walk in G, that uses an odd number of edges
*/

pub fn shortest_odd_walk<W: Weight, E: Edge<W>>(graph: &UndirectedGraph<W,E>, s: usize, t: usize) -> PathResult<W, E> {
    let n = graph.n();
    let mut even_dist: Vec<Cost<W>> = repeat(n, Infinite);
    let mut odd_dist = repeat(n, Infinite);
    even_dist[s] = Finite(0.into());
    let mut queue: BinaryHeap<(Reverse<Order<W>>, bool, usize)> = BinaryHeap::from([(Reverse(Order(0.into())), true, s)]);
    let mut even_prev: Vec<Option<&E>> = repeat(n, None);
    let mut odd_prev: Vec<Option<&E>> = repeat(n, None);
    let mut even_done: Vec<bool> = repeat(n, false);
    let mut odd_done: Vec<bool> = repeat(n, false);

    while let Some((Reverse(Order(dist_u)), even, u)) = queue.pop() {
        if even {
            if even_done[u] { continue }
            even_done[u] = true;
            for e in &graph[&u] {
                let dist_v = dist_u + e.weight();
                if Finite(dist_v) < odd_dist[e.to()] {
                    odd_dist[e.to()] = Finite(dist_v);
                    queue.push((Reverse(Order(dist_v)), false, e.to()));
                    odd_prev[e.to()] = Some(e);
                }
            }
        }
        else {
            if odd_done[u] { continue }
            odd_done[u] = true;
            for e in &graph[&u] {
                let dist_v = dist_u + e.weight();
                if Finite(dist_v) < even_dist[e.to()] {
                    even_dist[e.to()] = Finite(dist_v);
                    queue.push((Reverse(Order(dist_v)), true, e.to()));
                    even_prev[e.to()] = Some(e);
                }
            }
        }
        if odd_dist[t].is_finite() { break; }
    }

    match odd_dist[t] {
        Infinite => Impossible,
        Finite(cost) => {
            let mut path: Vec<E> = vec![odd_prev[t].unwrap().clone()];
            let mut v = path[0].from();
            while v != s {
                let e = even_prev[v].unwrap().clone();
                let o = odd_prev[e.from()].unwrap().clone();
                v = o.from();
                path.push(e);
                path.push(o);
            }
            path.reverse();
            Possible {
                cost,
                path,
            }
        }
    }
}
