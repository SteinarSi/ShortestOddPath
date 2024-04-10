use std::cmp::Reverse;
use crate::structure::path_result::{PathResult, PathResult::*};
use crate::structure::cost::{Cost::*, Cost};
use crate::structure::graph::graph::Graph;
use crate::structure::graph::undirected_graph::UndirectedGraph;
use crate::utility::misc::{debug, repeat};
use std::collections::{BinaryHeap, BTreeMap};
use crate::structure::graph::edge::{BasicEdge, Edge};
use crate::structure::todo::{Todo, Todo::*};
use crate::structure::weight::{Weight, Weighted};

pub struct DerigsAlgorithm<W: Weight> {
    graph: UndirectedGraph<W,BasicEdge<W>>,
    d_plus: Vec<Cost<W>>,
    d_minus: Vec<Cost<W>>,
    pred: Vec<Option<BasicEdge<W>>>,
    basis: Vec<usize>,
    bases: BTreeMap<usize, Vec<usize>>,
    s: usize,
    t: usize,
    orig_n: usize,
    completed: Vec<bool>,
    pq: BinaryHeap<Reverse<Todo<W>>>,
    in_current_cycle: Vec<bool>,
}

/**
Problem: Shortest Odd Path
In: an undirected graph G, two vertices s,t in V(G)
Out: the shortest s-t-path in G that uses an odd number of edges, if one exists.
*/

pub fn shortest_odd_path<W: Weight, E: Edge<W>>(graph: &UndirectedGraph<W,E>, s: usize, t: usize) -> PathResult<W,BasicEdge<W>> {
    DerigsAlgorithm::init(graph, s, t).solve()
}

impl <W: Weight> DerigsAlgorithm<W> {
    fn init<E: Edge<W>>(graph: &UndirectedGraph<W,E>, s: usize, t: usize) -> Self where Self: Sized {
        let mirror_graph = create_mirror_graph(graph, s, t);
        let n = mirror_graph.n();

        debug(format!("Looking for an odd {}-{}-path here:\n{:?}\n", s, t, mirror_graph));

        let mut d_plus= repeat(n, Infinite);
        let mut d_minus = repeat(n, Infinite);
        let mut pred = repeat(n, None);
        let mut completed = repeat(n, false);
        let mut pq = BinaryHeap::new();
        d_plus[s] = Finite(0.into());

        for e in &mirror_graph[&s] {
            pq.push(Reverse(Vertex(e.weight(), e.to())));
            d_minus[e.to()] = Finite(e.weight());
            pred[e.to()] = Some(e.clone());
        }
        completed[s] = true;
        completed[s + graph.n()] = true;

        DerigsAlgorithm {
            graph: mirror_graph,
            d_plus,
            d_minus,
            pred,
            basis: (0..n).collect(),
            bases: BTreeMap::new(),
            s,
            t,
            orig_n: graph.n(),
            completed,
            pq,
            in_current_cycle: repeat(n, false),
        }
    }

    fn solve(&mut self) -> PathResult<W,BasicEdge<W>> {
        if self.s == self.t {
            return Impossible;
        }

        while ! self.control() {}

        self.print_state();

        if self.d_minus[self.t].is_infinite() {
            debug(format!("\n\nWe can now definitely conclude that no odd {}-{}-path exists.\n\n", self.s, self.t));
            return Impossible;
        }

        debug(format!("\n\nAn {}-{}-path exists. Backtracking...", self.s, self.t));

        let mut curr = self.pred[self.t].clone().unwrap();
        let mut cost = curr.weight();
        let mut path = vec![curr.clone()];
        while curr.from() != self.s {
            curr = self.pred[self.mirror(curr.from())].clone().unwrap();
            cost = cost + curr.weight();
            path.push(BasicEdge::new(self.to_real_vertex(curr.from()), self.to_real_vertex(curr.to()), curr.weight()));
        }
        path.reverse();

        debug(format!("Path of cost {} is possible: {:?\n\n}",cost, path));
        Possible {
            cost,
            path,
        }
    }
    // Return true if the search is done. Either because we found the shortest odd s-t-path, or because none exist.
    fn control(&mut self) -> bool {
        self.print_state();

        while let Some(Reverse(todo)) = self.pq.peek() {
            match todo {
                Vertex(_,u) => if self.completed[*u] { self.pq.pop(); } else { break; }
                Blossom(_,u,v) => if self.basis[*u] == self.basis[*v] { self.pq.pop(); } else { break; }
            }
        }

        match self.pq.pop() {
            None => return true, // No odd path exists :(
            Some(Reverse(Vertex(delta, l))) => {
                if l == self.t { return true; } // Shortest odd path has been found :)
                self.grow(l, delta);
            }
            Some(Reverse(Blossom(delta, l, k))) => {
                self.blossom(l, k, delta);
            }
        }

        return false;
    }

    fn scan(&mut self, u: usize) {
        self.completed[u] = true;
        debug(format!("    Scan(k = {})", u));
        let dist_u = self.d_plus[u].expect(format!("        We called self.scan({}), but self.d_plus[{}] is undefined!", u, u).as_str());
        for e in &self.graph[&u] {
            let w = e.weight();
            let v = e.to();
            let new_dist_v = dist_u + w;
            if ! self.completed[v] {
                if Finite(new_dist_v) >= self.d_minus[v] { continue }

                debug(format!("        d_minus[{}] = {}", v, new_dist_v));
                debug(format!("        pred[{}] = {}", v, u));
                self.d_minus[v] = Finite(new_dist_v);
                // self.pred[v] = Some((u,w));
                // TODO ikke klone hele tiden
                self.pred[v] = Some(e.clone());
                self.pq.push(Reverse(Vertex(new_dist_v, v)));
            }

            else if let (Finite(dist_v), true) = (self.d_plus[v], self.basis[u] != self.basis[v]){
                debug(format!("        Found candidate for blossom: ({}, {}), with delta = {}", u, v, dist_u + dist_v + w));
                self.pq.push(Reverse(Blossom(dist_u + dist_v + w, u, v)));
                if Finite(new_dist_v) < self.d_minus[v] {
                    debug(format!("        d_minus[{}] = {}", v, new_dist_v));
                    debug(format!("        pred[{}] = {}", v, u));
                    self.d_minus[v] = Finite(new_dist_v);
                    // self.pred[v] = Some((u,w));
                    self.pred[v] = Some(e.clone());
                }
            }
        }
    }

    fn grow(&mut self, l: usize, delta: W) {
        debug(format!("Grow(l = {}, delta = {})", l, delta));
        let k = self.mirror(l);
        self.d_plus[k] = Finite(delta);
        self.scan(k);
    }

    fn blossom(&mut self, l: usize, k: usize, delta: W) {
        debug(format!("Blossom(l = {}, k = {}, delta = {}):", l, k, delta));

        let (b, p1, p2) = self.backtrack(l, k);
        debug(format!("    p1: {:?}", p1));
        debug(format!("    p2: {:?}", p2));
        self.set_bases(b, &p1);
        self.set_bases(b, &p2);

        self.set_cycle_path_values(&p1);
        self.set_cycle_path_values(&p2);
    }

    fn backtrack(&mut self, l: usize, k: usize) -> (usize, Vec<(usize, (usize, W))>, Vec<(usize, (usize, W))>) {
        // TODO ny måte å backtracke på
        let p1e = self.pred[l].clone().unwrap();
        let p2e = self.pred[k].clone().unwrap();
        let mut p1 = vec![(self.basis[l], (p1e.from(), p1e.weight()))];
        let mut p2 = vec![(self.basis[k], (p2e.from(), p2e.weight()))];

        let mut u = self.basis[l];
        let mut v = self.basis[k];

        self.in_current_cycle[u] = true;
        self.in_current_cycle[v] = true;

        debug(format!("    Starting to backtrack from (l, k) = ({}, {}):", l, k));

        loop {
            debug(format!("        u = {}, v = {}", u, v));
            debug(format!("        p1 = {:?}", p1));
            debug(format!("        p2 = {:?}", p2));
            if u != self.s {
                p1.push((self.basis[self.mirror(u)], (u, 0.into())));
                u = self.basis[self.mirror(u)];
                self.in_current_cycle[u] = true;
                // let (mut uu, w) = self.pred[u].expect(format!("Tried to find pred[{}], but it was None! \nSo far we had this: \np1 = {:?}", u, p1).as_str());
                let e = self.pred[u].clone().expect(format!("Tried to find pred[{}], but it was None! \nSo far we had this: \np1 = {:?}", u, p1).as_str());
                let mut uu = e.from();
                let w = e.weight();
                uu = self.basis[uu];
                if self.in_current_cycle[uu] {
                    debug(format!("        Found b = {}", uu));
                    self.in_current_cycle[uu] = false;
                    while let Some((vv,_)) = p2.last() {
                        self.in_current_cycle[*vv] = false;
                        if vv == &uu {
                            p2.pop();
                            break;
                        }
                        p2.pop();
                    }
                    return (uu, p1, p2);
                }
                p1.push((uu, (u, w)));
                u = uu;
                self.in_current_cycle[u] = true;
            }
            if v != self.s {
                p2.push((self.basis[self.mirror(v)], (v, 0.into())));
                v = self.basis[self.mirror(v)];
                self.in_current_cycle[v] = true;
                // let (mut vv, w) = self.pred[v].expect(format!("Tried to find pred[{}], but it was None! \nSo far we had this: \np2 = {:?}", v, p2).as_str());
                let e = self.pred[v].clone().expect(format!("Tried to find pred[{}], but it was None! \nSo far we had this: \np2 = {:?}", v, p2).as_str());
                let mut vv = e.from();
                let w = e.weight();
                vv = self.basis[vv];
                if self.in_current_cycle[vv] {
                    debug(format!("        Found b = {}", vv));
                    self.in_current_cycle[vv] = false;
                    while let Some((uu,_)) = p1.last() {
                        self.in_current_cycle[*uu] = false;
                        if uu == &vv {
                            p1.pop();
                            break;
                        }
                        p1.pop();
                    }
                    return (vv, p1, p2);
                }
                p2.push((vv, (v, w)));
                v = vv;
                self.in_current_cycle[v] = true;
            }
        }
    }

    fn set_bases(&mut self, b: usize, path: &Vec<(usize, (usize, W))>) {
        let mut ex = Vec::new();
        for &(u,_) in path {
            if self.basis[u] != b {
                self.basis[u] = b;
                ex.push(u);
                if let Some(xs) = self.bases.get(&u) {
                    for &v in xs {
                        self.basis[v] = b;
                        ex.push(v);
                    }
                }
            }
        }
        self.bases.entry(b).or_default().extend(ex);
    }

    fn set_cycle_path_values(&mut self, path: &Vec<(usize, (usize, W))>) {
        for &(u, (v, w)) in path {
            self.in_current_cycle[u] = false;
            if self.d_minus[v] + Finite(w) < self.d_plus[u] {
                debug(format!("    {} is not outer, so self.d_plus[{}] = {:?} + {:?} = {:?} now. The old was {:?}", u, u, self.d_minus[v], Finite(w), self.d_minus[v] + Finite(w), self.d_plus[u]));
                self.d_plus[u] = self.d_minus[v] + Finite(w);
                self.scan(u);
            }
            if self.d_plus[v] + Finite(w) < self.d_minus[u] {
                debug(format!("    Found a better d_minus[{}], {:?} vs the old {:?}", u, self.d_plus[v] + Finite(w), self.d_minus[u]));
                self.d_minus[u] = self.d_plus[v] + Finite(w);
                // self.pred[u] = Some((v, w));
                self.pred[u] = Some(BasicEdge::new(v, u, w));
                // TODO ny måte å backtracke på
            }
        }
    }

    fn mirror(&self, u: usize) -> usize {
        if u < self.orig_n {
            u + self.orig_n
        } else {
            u - self.orig_n
        }
    }

    fn to_real_vertex(&self, u: usize) -> usize {
        if u >= self.orig_n {
            u - self.orig_n
        }
        else {
            u
        }
    }

    fn print_state(&self) {
        let mut pq = self.pq.clone().into_vec();
        pq.sort();
        pq.reverse();
        debug("State:".to_string());
        debug(format!("    d_plus:  {:?}", self.d_plus));
        debug(format!("    d_minus: {:?}", self.d_minus));
        debug(format!("    pred: {:?}", self.pred));
        debug(format!("    PQ: {:?}", pq));
        debug(format!("    Completed: {:?}", self.graph.vertices().into_iter().filter(|&u| self.completed[u]).collect::<Vec<usize>>()));
        debug(format!("    Current cycle: {:?}", self.in_current_cycle));
    }
}

fn create_mirror_graph<W: Weight,E: Edge<W>>(graph: &UndirectedGraph<W,E>, s: usize, t: usize) -> UndirectedGraph<W,BasicEdge<W>> {
    let orig_n = graph.n();
    let new_n = orig_n * 2;
    let mut mirror = UndirectedGraph::new(new_n);
    for u in graph.vertices() {
        mirror[&u] = graph[&u].iter()
            .map(|e| BasicEdge::new(e.from(), e.to(), e.weight()))
            .collect();
        if u != s && u != t {
            mirror[&(u + orig_n)] = graph[&u].iter()
                .filter(|e| e.to() != s && e.to() != t)
                .map(|e| BasicEdge::new(u + orig_n, e.to() + orig_n, e.weight()))
                .collect()
        }
    }
    mirror
}
