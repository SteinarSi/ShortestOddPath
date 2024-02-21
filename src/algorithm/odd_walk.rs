use crate::structure::{
    undirected_graph::UndirectedGraph,
    graph::Graph
};
use std::collections::VecDeque;
use std::iter;
use crate::algorithm::algorithm::{Algorithm, PathResult, PathResult::*, ShortestOddWalk};
use crate::structure::cost::{Cost, Finite, Infinite};

pub struct BasicOddWalk {
    start: usize,
    end: usize,
    graph: UndirectedGraph,
    odd_dist: Vec<Cost>,
    even_dist: Vec<Cost>,
    queue: VecDeque<(usize, u64)>,
}

impl Algorithm for BasicOddWalk {
    type Pr = ShortestOddWalk;

    fn init((graph, start, end): (UndirectedGraph, usize, usize)) -> Self where Self: Sized {
        let mut even_dist: Vec<Cost> = iter::repeat_with(|| Infinite).take(graph.n()).collect();
        let n = graph.n();
        even_dist[start] = Finite(0);
        BasicOddWalk {
            start,
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
