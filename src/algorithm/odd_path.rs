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
    pred: Vec<Option<usize>>,
    basis: Vec<usize>,
    bases: BTreeMap<usize, Vec<usize>>,
    s: usize,
    t: usize,
    orig_n: usize,
    completed: Vec<bool>,
    pq: BinaryHeap<Todo<W>>,
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
        // TODO use w for something
        for &(w, v) in mirror_graph.neighbourhood(&s) {
            pq.push(Vertex(w, v));
            d_minus[v] = Finite(w);
            pred[v] = Some(s);
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

            // TODO finne den faktiske kostnaden
            cost = cost + 1.into();
            current = self.pred[current].expect(format!("    Tried to backtrack and find the path, but self.pred[{}] was undefined!", current).as_str());
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

        while let Some(todo) = self.pq.peek() {
            match todo {
                Vertex(_,u) => if self.completed[*u] { self.pq.pop(); } else { break; }
                Blossom(_,u,v) => if self.basis[*u] == self.basis[*v] { self.pq.pop(); } else { break; }
            }
        }

        match self.pq.pop() {
            None => return true, // No odd path exists :(
            Some(Vertex(delta, l)) => {
                if l == self.t { return true; } // Shortest odd path has been found :)
                self.grow(l, delta);
            }
            Some(Blossom(delta, l, k)) => {
                self.blossom(l, k, delta);
            }
        }

        return false;
    }

    fn scan(&mut self, u: usize) {
        self.completed[u] = true;
        debug(format!("    Scan(k = {}", u));
        let dist_u = self.d_plus[u].expect(format!("        We called self.scan({}), but self.d_plus[{}] is undefined!", u, u).as_str());
        // TODO use w
        for &(w, v) in self.graph.neighbourhood(&u) {
            let new_dist_v = dist_u + w;
            if ! self.completed[v] {
                if Finite(new_dist_v) >= self.d_minus[v] { continue }

                debug(format!("        d_minus[{}] = {}", v, new_dist_v));
                debug(format!("        pred[{}] = {}", v, u));
                self.d_minus[v] = Finite(new_dist_v);
                self.pred[v] = Some(u);
                self.pq.push(Vertex(new_dist_v, v));
            }

            else if let (Finite(dist_v), true) = (self.d_plus[v], self.basis[u] != self.basis[v]){
                debug(format!("        Found candidate for blossom: ({}, {}), with delta = {}", u, v, (dist_u + dist_v + w) / 2.into()));
                // TODO bytte med w for vektet
                self.pq.push(Blossom(dist_u + dist_v + w, u, v));
                if Finite(new_dist_v) < self.d_minus[v] {
                    self.d_minus[v] = Finite(new_dist_v);
                    self.pred[v] = Some(u);
                }
            }
        }
    }

    fn grow(&mut self, l: usize, delta: W) {
        debug(format!("Grow(l = {}, delta = {})", l, delta));
        let k = self.mirror(l);
        debug(format!("    n = {}, mirror({}) = {}", self.graph.n(), l, k));
        self.d_plus[k] = Finite(delta);
        self.scan(k);
    }

    fn blossom(&mut self, l: usize, k: usize, delta: W) {
        debug(format!("Blossom(l = {}, k = {}, delta = {}):", l, k, delta));

        let (b, p1, p2) = self.backtrack_cycle(l, k);

        self.set_bases(b, &p1);
        self.set_bases(b, &p2);

        // TODO erstatt 1 med vekten mellom l og k for vektet
        // TODO veldig stygt, men funker for nå. Bør antageligvis lagre verdien på kanten eller deltaen i PQQ eller noe. Men uten å dele på 2.
        let (w, _) = self.graph[l].iter().find(|(_,vv)| &k == vv).unwrap();
        let two_delta = self.d_plus[l] + self.d_plus[k] + Finite(*w);
        self.set_cycle_path_values(&p1, two_delta);
        self.set_cycle_path_values(&p2, two_delta);
    }

    fn backtrack_cycle(&self, l: usize, k: usize) -> (usize, Vec<usize>, Vec<usize>) {
        let mut u = self.basis[l];
        let mut v= self.basis[k];
        if self.d_plus[u] < self.d_plus[v] {
            u = self.basis[k];
            v = self.basis[l];
        }
        let mut p1 = Vec::new();
        let mut p2 = Vec::new();
        while self.d_plus[u] > self.d_plus[v] {
            debug(format!("self.d_plus[{}] = {:?} > self.d_plus[{}] = {:?}", l, self.d_plus[u], v, self.d_plus[v]));
            p1.push(u);
            u = self.basis[self.mirror(u)];
            debug(format!("u = {}", u));
            p1.push(u);
            u = self.basis[self.pred[u].expect(format!("Tried to find pred[{}], but it was None! \nSo far we had this: \n p1 = {:?}", u, p1).as_str())];
            debug(format!("u = {}", u));
        }
        debug(format!("After the loop we have {} -> {}, {} -> {}, where d_plus[{}] = {:?}, d_plus[{}] = {:?}", l, u, k, v, u, self.d_plus[u], v, self.d_plus[v]));
        while u != v {
            if u != self.s {
                p1.push(u);
                u = self.basis[self.mirror(u)];
                debug(format!("first u = {}, with d_minus[{}] = {:?}", u, u, self.d_minus[u]));
                p1.push(u);
                u = self.basis[self.pred[u].expect(format!("Tried to find pred[{}], but it was None! \nSo far we had this: \np1 = {:?}\np2 = {:?}", u, p1, p2).as_str())];
                debug(format!("second u = {}, with d_plus[{}] = {:?}", u, u, self.d_plus[u]));
            }
            else {
                debug("u has reached s, halting for now".to_string());
            }
            if v != self.s {
                p2.push(v);
                v = self.basis[self.mirror(v)];
                debug(format!("first v = {}, with d_minus[{}] = {:?}", v, v, self.d_minus[v]));
                p2.push(v);
                v = self.basis[self.pred[v].expect(format!("Tried to find pred[{}], but it was None! \nSo far we had this: \np1 = {:?}\np2 = {:?}", v, p1, p2).as_str())];
                debug(format!("second v = {}, with d_plus[{}] = {:?}", v, v, self.d_plus[v]));
            }
            else {
                debug("v has reached s, halting for now".to_string());
            }
        }
        debug(format!("b = {}\np1 = {:?}, \np2 = {:?}", u, p1, p2));
        return (u, p1, p2);
    }

    fn set_bases(&mut self, b: usize, path: &Vec<usize>) {
        let mut ex = Vec::new();
        for &u in path {
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

    fn set_cycle_path_values(&mut self, path: &Vec<usize>, two_delta: Cost<W>) {
        for i in 0..path.len() {
            let u = path[i];
            if ! self.is_outer(u) {
                self.d_plus[u] = two_delta - self.d_minus[u];
                debug(format!("    {} is not outer, so self.d_plus[{}] = {} now", u, u, self.d_plus[u].unwrap()));
                self.scan(u);
            }
            if i < path.len()-1 {
                let v = path[i + 1];
                // TODO erstatt med w for vektet
                // TODO oooooooooooooooooooooooooooooooooooooooooooooooo
                if self.d_plus[u] + Finite(1.into()) < self.d_minus[v] {
                    // TODO aaaaaaaaaaaaaaaaaaaaaa
                    debug(format!("    {} has no prev, so d_minus[{}] = {:?}, pred[{}] = {} now", v, v, self.d_plus[u] + Finite(1.into()), v, u));
                    // TODO eeeeeeeeeeeeeeeeeeeeeeeeeee
                    self.d_minus[v] = self.d_plus[u] + Finite(1.into());
                    self.pred[v] = Some(u);
                }
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
        debug("State:".to_string());
        debug(format!("    d_plus:  {:?}", self.d_plus));
        debug(format!("    d_minus: {:?}", self.d_minus));
        debug(format!("    pred: {:?}", self.pred));
        debug(format!("    PQ: {:?}", pq));
        debug(format!("    Completed: {:?}", self.graph.vertices().into_iter().filter(|&u| self.completed[u]).collect::<Vec<usize>>()));
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
