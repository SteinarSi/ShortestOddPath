use crate::algorithm::algorithm::{Algorithm, PathResult, Problem, ShortestOddPath};
use crate::algorithm::algorithm::PathResult::{Impossible, Possible};
use crate::structure::cost::{Cost::*, Cost};
use crate::structure::graph::Graph;
use crate::structure::rooted_tree::RootedTree;
use crate::structure::undirected_graph::UndirectedGraph;
use crate::utility::misc::{debug, repeat};

pub struct DerigsAlgorithm {
    graph: UndirectedGraph,
    d_plus: Vec<Cost>,
    d_minus: Vec<Cost>,
    pred: Vec<Option<usize>>,
    basis: Vec<usize>,
    s: usize,
    t: usize,
    orig_n: usize,
    path_tree: RootedTree,
    completed: Vec<bool>,

    scanned: Vec<usize>,
}

pub fn shortest_even_path(graph: UndirectedGraph, s: usize, t: usize) -> PathResult {
    DerigsAlgorithm::init((graph, s, t)).solve()
}

impl Algorithm for DerigsAlgorithm {
    type Pr = ShortestOddPath;

    fn init((graph, s, t): <Self::Pr as Problem>::In) -> Self where Self: Sized {
        let mirror_graph = create_mirror_graph(&graph, s, t);
        let n = mirror_graph.n();

        debug(format!("Looking for an odd {}-{}-path here:\n{:?}\n", s, t, mirror_graph));

        let mut d_plus = repeat(n, Infinite);
        let mut d_minus = repeat(n, Infinite);
        let mut pred = repeat(n, None);
        let mut completed = repeat(n, false);

        d_plus[s] = Finite(0);
        for &v in mirror_graph.neighbourhood(&s) {
            d_minus[v] = Finite(1); // Bytt med w for weighted
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
            s,
            t,
            orig_n: graph.n(),
            path_tree: RootedTree::new(s, n),
            completed,

            scanned: Vec::new(),
        }
    }

    fn solve(&mut self) -> <Self::Pr as Problem>::Out {
        // TODO er det riktig å bare loope her, eller er det tilfeller der Control ikke er sjefen?
        let mut DEBUG = 20;
        while ! self.control() {
            DEBUG -= 1;
            if DEBUG == 0 {
                panic!("\nTimeout!\n");
            }
        }

        self.print_state();

        if self.d_minus[self.t].is_infinite() {
            debug(format!("\n\nWe can now definitely conclude that no odd {}-{}-path exists.\n\n", self.s, self.t));
            return Impossible;
        }

        debug("\n\nAn s-t-path exists. Backtracking...".to_string());

        let mut cost = 0;
        let mut current = self.t;
        let mut path = vec![self.t];

        while current != self.mirror(self.s) {
            debug(format!("    current: {}", current));
            cost += 1;
            current = self.pred[current].expect(format!("    Tried to backtrack and find the path, but self.pred[{}] was undefined!", self.mirror(current)).as_str());
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
}
impl DerigsAlgorithm {
    // Return true if the search is done. Either because we found the shortest odd s-t-path, or because none exist.
    fn control(&mut self) -> bool {
        self.print_state();
        let d1 = self.graph
            .vertices().into_iter()
            .filter(|&u| ! self.completed[u] && self.d_minus[u].is_finite())
            .map(|u| (self.d_minus[u].unwrap(), u))
            .min();
        let d2 = self.graph
            .vertices().into_iter()
            .filter(|&u| self.d_plus[u].is_finite() && self.d_minus[u].is_finite())
            // TODO trenger en bedre måte å sammenligne dette
            .min_by(|&u, &v| {
                let a = (self.d_minus[u].unwrap() + self.d_plus[u].unwrap()) as f64 / 2.0;
                let b = (self.d_minus[v].unwrap() + self.d_plus[v].unwrap()) as f64 / 2.0;
                a.total_cmp(&b)
            })
            .map(|u| ((self.d_minus[u].unwrap() + self.d_plus[u].unwrap()) / 2, u));

        debug(format!("Control: \n    d1 := {:?}\n    d2 := {:?}", d1, d2));

        // Find the lowest delta, but also take into account that they may not be defined. Since None < Some(0), we can't just use the built-in comparison.
        let (delta, l) = match (d1, d2) {
            (None, None) => { return true; } // No odd path exists :(
            (Some((delta_1, u)), Some((delta_2, v))) => {
                if delta_1 <= delta_2 { (delta_1, u) }
                else {(delta_2, v)}
            },
            (Some(x), None) => {x}
            (None, Some(x)) => {x}
        };
        debug(format!("    (delta, l) := ({}, {})", delta, l));
        if Some((delta, l)) == d1 {
            if l == self.t { return true } // shortest odd path has been found, we can quit now
            self.grow(l, delta);
        }
        else {
            self.blossom(l, delta);
        }

        return false;
    }

    fn scan(&mut self, u: usize, bans: &Vec<usize>) {
        debug(format!("    Scan(k = {})", u));
        self.scanned.push(u);
        let dist_u = self.d_plus[u].expect(format!("        We called self.scan({}), but self.d_plus[{}] is undefined!", u, u).as_str());
        for &v in self.graph.neighbourhood(&u) {
            if bans.contains(&v) { continue }
            if let Finite(dist_v) = self.d_minus[v] {
                // swap with w for the weighted case
                // TODO infeffektivt, fiks senere
                if dist_u + 1 >= dist_v { continue }
            }
            debug(format!("        d_minus[{}] = {}", v, dist_u+1));
            debug(format!("        pred[{}] = {}", v, u));
            self.d_minus[v] = Finite(dist_u + 1);
            self.pred[v] = Some(u);
        }
    }

    fn grow(&mut self, l: usize, delta: u64) {
        debug(format!("Grow(l = {}, delta = {})", l, delta));
        let k = self.mirror(l);

        debug(format!("    n = {}, mirror({}) = {}", self.graph.n(), l, k));

        // TODO Trenger kanskje bare å legge til den ene? Sjekk hvilken, og om det er (l, k) eller (k, l).
        self.path_tree.add_edge(k, l);
        self.completed[k] = true;
        self.completed[l] = true;

        // TODO Pål sier dette er en typo i papiret, skulle vært d_plus
        // self.d_minus[k] = Finite(delta);
        // TODO dette må seriøst dobbelsjekkes
        self.d_plus[k] = Finite(delta);

        self.scan(k, &Vec::new());
    }

    fn blossom(&mut self, l: usize, delta: u64) {
        let (b, cycle) = self.find_cycle_base(l);
        debug(format!("Blossom(l = {}, delta = {}), with b = {}", l, delta, b));

        // TODO Veldig inneffektivt, men fungerer som en MVP
        for u in self.graph.vertices() {
            if cycle.contains(&u) {
                self.basis[u] = b;
            }
        }
        for &u in &cycle {
            if ! self.is_outer(u) {
                self.d_plus[u] = Finite(2 * delta) - self.d_minus[u];
                debug(format!("    {} is not outer, so self.d_plus[{}] = {} now", u, u, self.d_plus[u].unwrap()));
                self.scan(u, &cycle);
            }
        }
    }

    // TODO hele er ekstremt ineffektiv, fiks senere
    fn find_cycle_base(&self, l: usize) -> (usize, Vec<usize>){
        let k = self.pred[l].expect(format!("        Tried to set k := pred[{}], but it isn't defined!", l).as_str());
        debug(format!("    Finding cycle starting at l = {}, k = {}:", l, k));

        let p1 = self.find_path(l);
        let p2 = self.find_path(self.mirror(l));

        debug(format!("        path1: {:?}", p1));
        debug(format!("        path2: {:?}", p2));

        let b = *p1.iter().find(|&u| p2.contains(u)).unwrap();

        debug(format!("        b: {}", b));
        let mut ret: Vec<usize> = Vec::new();
        ret.push(b);
        ret.extend(&p1[..p1.iter().position(|&u| u == b).unwrap()]);
        ret.extend(&p2[..p2.iter().position(|&u| u == b).unwrap()]);

        println!("        Cycle: {:?}", ret);

        (b, ret)
    }

    fn find_path(&self, mut u: usize) -> Vec<usize> {
        let mut ret = vec![u];
        loop {
            if u == self.s { break; }
            u = self.mirror(u);
            ret.push(u);

            if u == self.s { break; }
            u = self.pred[u].unwrap();
            ret.push(u);
        }
        ret
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
        debug("State:".to_string());
        debug(format!("    scanned: {:?}", self.scanned));
        debug(format!("    d_plus: {:?}", self.d_plus));
        debug(format!("    d_minus: {:?}", self.d_minus));
        debug(format!("    pred: {:?}", self.pred));
    }
}

fn create_mirror_graph(graph: &UndirectedGraph, s: usize, t: usize) -> UndirectedGraph {
    let orig_n = graph.n();
    let new_n = orig_n * 2;
    let mut mirror = UndirectedGraph::new(new_n);
    for u in graph.vertices() {
        mirror.set_neighbourhood(u, graph[u].clone());
        if u != s && u != t {
            mirror.set_neighbourhood(u + orig_n,
                 graph[u].iter()
                .filter(|&&v| v != s && v != t)
                .map(|v| v + orig_n)
                .collect());
        }
    }
    mirror
}


#[cfg(test)]
mod test_odd_path {
    use crate::utility::testing::{test_s_t_trip};
    use super::*;

    fn test_path(folder: &str, file: &str) {
        debug(format!("{}/{}", folder, file));
        test_s_t_trip::<DerigsAlgorithm>(folder, file)
    }
    #[test]
    fn test_basic_paths() {
        test_path("small_graphs", "small1");
        test_path("small_graphs", "small2");
        test_path("small_graphs", "small3");
        test_path("small_graphs", "small4");
    }
}