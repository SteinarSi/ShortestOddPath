use std::cmp::Reverse;
use crate::structure::path_result::{PathResult, PathResult::*};
use crate::structure::cost::{Cost::*, Cost};
use crate::structure::graph::Graph;
use crate::structure::undirected_graph::UndirectedGraph;
use crate::utility::misc::{debug, repeat};
use std::collections::{BinaryHeap, BTreeMap};
use crate::structure::todo::{Todo, Todo::*};
use crate::structure::weight::Weight;

pub struct DerigsAlgorithm<W: Weight> {
    graph: UndirectedGraph<W>,
    d_plus: Vec<Cost<W>>,
    d_minus: Vec<Cost<W>>,
    pred: Vec<Option<(usize,W)>>,
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

pub fn shortest_odd_path<W: Weight>(graph: &UndirectedGraph<W>, s: usize, t: usize) -> PathResult<W> {
    DerigsAlgorithm::init(graph, s, t).solve()
}

impl <W: Weight> DerigsAlgorithm<W> {
    fn init(graph: &UndirectedGraph<W>, s: usize, t: usize) -> Self where Self: Sized {
        let mirror_graph = create_mirror_graph(graph, s, t);
        let n = mirror_graph.n();

        debug(format!("Looking for an odd {}-{}-path here:\n{:?}\n", s, t, mirror_graph));

        let mut d_plus= repeat(n, Infinite);
        let mut d_minus = repeat(n, Infinite);
        let mut pred = repeat(n, None);
        let mut completed = repeat(n, false);
        let mut pq = BinaryHeap::new();
        d_plus[s] = Finite(0.into());

        for &(w, v) in mirror_graph.neighbourhood(&s) {
            pq.push(Reverse(Vertex(w, v)));
            d_minus[v] = Finite(w);
            pred[v] = Some((s,w));
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

    fn solve(&mut self) -> PathResult<W> {
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

        let mut cost: W = 0.into();
        let mut current = self.t;
        let mut path = vec![self.t];

        while current != self.mirror(self.s) {
            debug(format!("    current: {}", current));

            let (v, w) = self.pred[current].expect(format!("    Tried to backtrack and find the path, but self.pred[{}] was undefined!", current).as_str());
            current = v;
            cost = cost + w;
            debug(format!("    current: {}", current));
            path.push(current);
            current = self.mirror(current);
        }
        path.reverse();
        path = path.into_iter().map(|u| if u >= self.orig_n { u - self.orig_n } else {u} ).collect();

        debug(format!("Path of cost {} is possible: {:?\n\n}",cost, path));

        Possible {
            cost,
            path
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
        for &(w, v) in self.graph.neighbourhood(&u) {
            let new_dist_v = dist_u + w;
            if ! self.completed[v] {
                if Finite(new_dist_v) >= self.d_minus[v] { continue }

                debug(format!("        d_minus[{}] = {}", v, new_dist_v));
                debug(format!("        pred[{}] = {}", v, u));
                self.d_minus[v] = Finite(new_dist_v);
                self.pred[v] = Some((u,w));
                self.pq.push(Reverse(Vertex(new_dist_v, v)));
            }

            else if let (Finite(dist_v), true) = (self.d_plus[v], self.basis[u] != self.basis[v]){
                debug(format!("        Found candidate for blossom: ({}, {}), with delta = {}", u, v, dist_u + dist_v + w));
                self.pq.push(Reverse(Blossom(dist_u + dist_v + w, u, v)));
                if Finite(new_dist_v) < self.d_minus[v] {
                    debug(format!("        d_minus[{}] = {}", v, new_dist_v));
                    debug(format!("        pred[{}] = {}", v, u));
                    self.d_minus[v] = Finite(new_dist_v);
                    self.pred[v] = Some((u,w));
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

        self.set_cycle_path_values(&p1, delta);
        self.set_cycle_path_values(&p2, delta);
    }

    fn backtrack(&mut self, l: usize, k: usize) -> (usize, Vec<(usize, (usize, W))>, Vec<(usize, (usize, W))>) {
        let mut p1 = vec![(self.basis[l], self.pred[l].unwrap())];
        let mut p2 = vec![(self.basis[k], self.pred[k].unwrap())];

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
                let (mut uu, w) = self.pred[u].expect(format!("Tried to find pred[{}], but it was None! \nSo far we had this: \np1 = {:?}", u, p1).as_str());
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
                let (mut vv, w) = self.pred[v].expect(format!("Tried to find pred[{}], but it was None! \nSo far we had this: \np2 = {:?}", v, p2).as_str());
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

    fn set_cycle_path_values(&mut self, path: &Vec<(usize, (usize, W))>, delta: W) {
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
                self.pred[u] = Some((v, w));
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

    fn is_outer(&self, u: usize) -> bool {
        self.d_plus[u].is_finite()
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

fn create_mirror_graph<W: Weight>(graph: &UndirectedGraph<W>, s: usize, t: usize) -> UndirectedGraph<W> {
    let orig_n = graph.n();
    let new_n = orig_n * 2;
    let mut mirror = UndirectedGraph::new(new_n);
    for u in graph.vertices() {
        mirror[u] = graph[u].clone();
        if u != s && u != t {
            mirror[u + orig_n] = graph[u].iter()
                .filter(|&&(_, v)| v != s && v != t)
                .map(|&(w, v)| (w, v + orig_n))
                .collect()
        }
    }
    mirror
}
