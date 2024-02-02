use crate::algorithm::algorithm::{Algorithm, PathResult, Problem, ShortestOddPath};
use crate::algorithm::algorithm::PathResult::{Impossible, Possible};
use crate::structure::cost::{Cost::*, Cost};
use crate::structure::graph::Graph;
use crate::structure::rooted_tree::RootedTree;
use crate::structure::undirected_graph::UndirectedGraph;
use crate::utility::misc::{debug, repeat};

struct DerigsAlgorithm {
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
}

pub fn shortest_even_path(graph: UndirectedGraph, s: usize, t: usize) -> PathResult {
    DerigsAlgorithm::init((graph, s, t)).solve()
}

impl Algorithm for DerigsAlgorithm {
    type Pr = ShortestOddPath;

    fn init((graph, s, t): <Self::Pr as Problem>::In) -> Self where Self: Sized {
        let mirror_graph = create_mirror_graph(&graph);
        let n = mirror_graph.n();

        let mut d_plus = repeat(n, Infinite);
        let mut d_minus = repeat(n, Infinite);
        let mut pred = repeat(n, None);
        let mut completed = repeat(n, false);

        // TODO temporarily set to s
        let s_prime = s;
        // let s_prime = s + graph.n();

        d_plus[s_prime] = Finite(0);
        for &v in mirror_graph.neighbourhood(&s_prime) {
            d_minus[v] = Finite(1); // Bytt med w for weighted
            pred[v] = Some(s_prime);
        }
        completed[s_prime] = true;


        DerigsAlgorithm {
            graph: mirror_graph,
            d_plus,
            d_minus,
            pred,
            basis: (0..n).collect(),
            s,
            t,
            orig_n: graph.n(),
            path_tree: RootedTree::new(s_prime, n),
            completed,
        }
    }

    fn solve(&mut self) -> <Self::Pr as Problem>::Out {
        // TODO er det riktig å bare loope her, eller er det tilfeller der Control ikke er sjefen?
        while self.control() {}

        if self.d_plus[self.t].is_infinite() {
            return Impossible;
        }

        let mut cost = 0;
        let mut parent = self.t;
        let mut path = vec![self.t];

        while parent != self.mirror(self.s) {
            cost += 1;
            parent = self.pred[self.mirror(parent)].expect(format!("Tried to backtrack and find the path, but self.pred[{}] was undefined!", self.mirror(parent)).as_str());
            path.push(parent);
        }
        path.reverse();

        Possible {
            cost,
            path
        }
    }
}
impl DerigsAlgorithm {
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
            .map(|u| ((self.d_minus[u].unwrap() + self.d_plus[u].unwrap()) / 2, u))
            .min();

        debug(format!("Control: \n    d1 := {:?}\n    d2 := {:?}", d1, d2));

        // Find the lowest delta, but also take into account that they may not be defined. Since None < Some(0), we can't just use the built-in comparison.
        let (delta, l) = match (d1, d2) {
            (None, None) => { return true; } // No even path exists :(
            (Some((delta_1, u)), Some((delta_2, v))) => {
                if delta_1 <= delta_2 { (delta_1, u) }
                else {(delta_2, v)}
            },
            (Some(x), None) => {x}
            (None, Some(x)) => {x}
        };
        debug(format!("    (delta, l) := ({}, {})", delta, l));
        if Some((delta, l)) == d1 {
            if l == self.t { return true } // shortest even path has been found, we can quit now
            self.grow(l, delta);
        }
        else {
            self.blossom(l, delta);
        }

        return false;
    }

    fn scan(&mut self, u: usize) {
        debug(format!("Scan({})", u));
        let dist_u = self.d_plus[u].expect(format!("We called self.scan({}), but self.d_plus[{}] is undefined!", u, u).as_str());
        for &v in self.graph.neighbourhood(&u) {
            if let Finite(dist_v) = self.d_minus[v] {
                // swap with w for the weighted case
                if dist_u + 1 >= dist_v { continue }

                self.d_minus[v] = Finite(dist_u + 1);
                self.pred[v] = Some(u);
            }
        }
    }

    fn grow(&mut self, l: usize, delta: u64) {
        debug(format!("Grow({}, {})", l, delta));
        let k = self.mirror(l);

        debug(format!("n = {}, mirror({}) = {}", self.graph.n(), l, k));

        // TODO Trenger kanskje bare å legge til den ene? Sjekk hvilken, og om det er (l, k) eller (k, l).
        self.path_tree.add_edge(k, l);
        self.completed[k] = true;
        self.completed[l] = true;
        self.d_minus[k] = Finite(delta);

        // TODO dette må seriøst dobbelsjekkes
        self.d_plus[k] = Finite(delta);

        self.scan(k);
    }

    fn blossom(&mut self, l: usize, delta: u64) {
        let (b, cycle) = self.find_cycle_base(l);
        debug(format!("Blossom({}, {}), with delta = {}", l, b, delta));

        // TODO Veldig inneffektivt, men fungerer som en MVP
        for u in self.graph.vertices() {
            if cycle.contains(&u) {
                self.basis[u] = b;
            }
        }
        for u in cycle {
            if self.d_plus[u].is_infinite() {
                // self.d_plus[u] = Some(2 * delta - self.d_minus[u].expect(format!("Tried to update d_plus[{}], but d_minus[{}] is undefined!", u, u).as_str()));
                self.d_plus[u] = Finite(2 * delta) - self.d_minus[u];
                self.scan(u);
            }
        }
    }

    fn find_cycle_base(&self, l: usize) -> (usize, Vec<usize>){
        let k = self.pred[l].expect(format!("Tried to set k := pred[{}], but it isn't defined!", l).as_str());
        let mut cycle = vec![l, k]; // Note that the cycle is NOT in order here
        let mut parent1 = l;
        let mut parent2 = k;
        let b = loop {
            parent1 = self.mirror(parent1);
            parent2 = self.mirror(parent2);
            cycle.push(parent1);
            if parent1 == parent2 { break parent1; }
            cycle.push(parent2);

            parent1 = self.pred[parent1].expect(format!("Tried to find the parent1 of {} in the cycle spawned by (l, k) = ({}, {}), but it was undefined!", parent1, l, k).as_str());
            parent2 = self.pred[parent2].expect(format!("Tried to find the parent2 of {} in the cycle spawned by (l, k) = ({}, {}), but it was undefined!", parent2, l, k).as_str());
            cycle.push(parent1);
            if parent1 == parent2 { break parent1; } // Dette skjer vel aldri? Med mindre b er matchet med to forskjellige noder. Hmm.
            cycle.push(parent2);
        };
        (b, cycle)
    }

    fn mirror(&self, u: usize) -> usize {
        if u < self.orig_n {
            u + self.orig_n
        } else {
            u - self.orig_n
        }
    }

    fn print_state(&self) {
        debug(format!("d_plus: {:?}", self.d_plus));
        debug(format!("d_minus: {:?}", self.d_minus));
    }
}

fn create_mirror_graph(graph: &UndirectedGraph) -> UndirectedGraph {
    let orig_n = graph.n();
    let new_n = orig_n * 2;
    let mut mirror = UndirectedGraph::new(new_n);
    for u in graph.vertices() {
        mirror.set_neighbourhood(u, graph[u].clone());
        mirror.set_neighbourhood(u + orig_n, graph[u].iter().map(|u| u + orig_n).collect());
    }
    mirror
}
