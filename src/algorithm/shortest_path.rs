use std::cmp::{Reverse};
use std::collections::BinaryHeap;
use queues::{IsQueue, Queue};
use crate::structure::cost::{Cost, Cost::*};
use crate::structure::graph::edge::Edge;
use crate::structure::graph::graph::Graph;
use crate::structure::graph::undirected_graph::UndirectedGraph;
use crate::structure::weight::{Order, Weight};
use crate::utility::misc::repeat;

pub fn all_shortest_paths<W: Weight, E: Edge<W>>(graph: &UndirectedGraph<W,E>, s: usize) -> Vec<Cost<W>> {
    let mut dist = repeat(graph.n(), Infinite);
    let mut done = repeat(graph.n(), false);
    dist[s] = Finite(0.into());
    let mut pqv: BinaryHeap<(Reverse<Order<W>>, usize)> = BinaryHeap::from([(Reverse(Order(0.into())), s)]);
    while let Some((Reverse(Order(d)), u)) = pqv.pop() {
        if ! done[u] {
            done[u] = true;
            for e in &graph[&u] {
                let v = e.to();
                let dv = d + e.weight();
                if Finite(dv) < dist[v] {
                    dist[v] = Finite(dv);
                    pqv.push((Reverse(Order(dv)), v));
                }
            }
        }

    }
    dist
}

pub fn bfs<W: Weight, E: Edge<W>>(graph: &UndirectedGraph<W,E>, s: usize) -> Vec<Cost<u64>> {
    let mut dist = repeat(graph.n(), Infinite);
    let mut q: Queue<(usize, u64)> = Queue::new();
    q.add((s, 0)).unwrap();
    dist[s] = Finite(0);

    while let Ok((u,d)) = q.remove() {
        for e in &graph[&u] {
            let v = e.to();
            if dist[v].is_infinite() {
                dist[v] = Finite(d + 1);
                q.add((v, d+1)).unwrap();
            }
        }
    }

    dist
}

#[cfg(test)]
mod find_worst_pairs {
    use crate::structure::graph::edge::BasicEdge;
    use super::*;

    /** Utility to find worst-case tests for each graph, for benchmarking purposes */
    fn find_worst(filename: &str) {
        let s = 0;
        let graph: UndirectedGraph<u64, BasicEdge<u64>> = std::fs::read_to_string(["data/real_graphs/", filename].concat()).unwrap().parse().unwrap();
        let dists = all_shortest_paths(&graph, s);
        let (cost, t) = (0..graph.n())
            .map(|u| (dists[u], u))
            .filter(|(d,_)| d.is_finite())
            .max()
            .unwrap();
        let seen: u64 = dists.iter()
            .filter(|c| c.is_finite())
            .map(|_| 1)
            .sum();
        println!("Distances: {:?}", dists);
        println!("Starting from {}, we can reach {} / {} vertices in the graph", s, seen, graph.n());
        println!("The by worst vertex to find from s = {} is {}, with a distance of {:?}.", s, t, cost)
    }
    #[test]
    fn find_worst_case_pair() {
        // find_worst("COX2.mtx");
        // find_worst("CityOfOldenburg.in");
    }
}
