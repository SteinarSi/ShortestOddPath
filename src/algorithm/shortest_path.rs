use std::cmp::{Reverse};
use std::collections::BinaryHeap;
use queues::{IsQueue, Queue};
use crate::structure::cost::{Cost, Cost::*};
use crate::structure::graph::edge::Edge;
use crate::structure::graph::undirected_graph::UndirectedGraph;
use crate::structure::path_result::PathResult;
use crate::structure::path_result::PathResult::{Impossible, Possible};
use crate::structure::weight::{Order, Weight};
use crate::utility::misc::repeat;

pub fn shortest_path<W: Weight, E: Edge<W>>(graph: &UndirectedGraph<W,E>, s: usize, t: usize) -> PathResult<W,E> {
    let mut dist = repeat(graph.n(), Infinite);
    let mut done = repeat(graph.n(), false);
    let mut prev: Vec<Option<E>> = repeat(graph.n(), None);
    dist[s] = Finite(0.into());
    let mut pqv: BinaryHeap<(Reverse<Order<W>>, usize)> = BinaryHeap::from([(Reverse(Order(0.into())), s)]);
    while let Some((Reverse(Order(d)), u)) = pqv.pop() {
        if ! done[u] {
            if u == t {
                let mut path = vec![prev[t].clone().unwrap()];
                while path.last().unwrap().from() != s {
                    path.push(prev[path.last().unwrap().from()].clone().unwrap());
                }
                path.reverse();
                return Possible {
                    path,
                    cost: dist[t].unwrap(),
                };
            }
            done[u] = true;
            for e in &graph[&u] {
                let v = e.to();
                let dv = d + e.weight();
                if Finite(dv) < dist[v] {
                    dist[v] = Finite(dv);
                    prev[v] = Some(e.clone());
                    pqv.push((Reverse(Order(dv)), v));
                }
            }
        }

    }
    return Impossible;
}

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

pub fn bfs<W,E>(graph: &UndirectedGraph<W,E>, s: usize) -> Vec<Cost<u64>>
    where W: Weight,
          E: Edge<W>,
{
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
