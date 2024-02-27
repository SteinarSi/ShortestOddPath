use crate::structure::path_result::{PathResult, PathResult::*};
use crate::structure::{
    undirected_graph::UndirectedGraph,
    graph::Graph
};
use std::collections::VecDeque;
use std::iter;
use crate::structure::cost::{Cost, Finite, Infinite};

pub struct BasicOddWalk {
    end: usize,
    graph: UndirectedGraph,
    odd_dist: Vec<Cost>,
    even_dist: Vec<Cost>,
    queue: VecDeque<(usize, u64)>,
}

/**
Problem: Shortest Odd Walk
In: an undirected graph G, and two vertices s,t in V(G)
Out: the shortest s-t-walk in G, that uses an odd number of edges
*/

pub fn shortest_odd_walk(graph: UndirectedGraph, s: usize, t: usize) -> PathResult {
    BasicOddWalk::init(graph, s, t).solve()
}

impl BasicOddWalk {

    fn init(graph: UndirectedGraph, start: usize, end: usize) -> Self {
        let mut even_dist: Vec<Cost> = iter::repeat_with(|| Infinite).take(graph.n()).collect();
        let n = graph.n();
        even_dist[start] = Finite(0);
        BasicOddWalk {
            end,
            graph,
            odd_dist: iter::repeat_with(|| Infinite).take(n).collect(),
            even_dist,
            queue: VecDeque::from([(start, 0)]),
        }
    }

    fn solve(&mut self) -> PathResult {
        while ! self.queue.is_empty() {
            let (u, distu) = self.queue.pop_front().unwrap();

            for &v in self.graph.neighbourhood(&u) {
                let distv = distu + 1;
                if distv % 2 == 0 && (self.even_dist[v].is_infinite() || self.even_dist[v].unwrap() > distv) {
                    self.even_dist[v] = Finite(distv);
                    self.queue.push_back((v, distv));
                }
                if distv % 2 == 1 && (self.odd_dist[v].is_infinite() || self.odd_dist[v].unwrap() > distv) {
                    self.odd_dist[v] = Finite(distv);
                    self.queue.push_back((v, distv));
                }
            }
            if self.odd_dist[self.end].is_finite() { break; }
        }

        match self.odd_dist[self.end] {
            Infinite => Impossible,
            Finite(cost) => Possible {
                cost,
                path: Vec::new(), // TODO finne den faktiske stien, om vi gidder
            }
        }
    }
}
