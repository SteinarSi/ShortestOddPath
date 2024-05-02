use queues::{IsQueue, Queue};
use crate::algorithm::odd_path::shortest_odd_path;
use crate::algorithm::utility::split_edges;
use crate::structure::graph::graph::Graph;
use crate::structure::path_result::{PathResult::*};
use crate::structure::graph::planar::planar_edge::PlanarEdge;
use crate::structure::graph::planar::planar_graph::PlanarGraph;
use crate::structure::weight::Weight;
use crate::utility::misc::repeat;

pub fn network_diversion<W: Weight>(graph: &PlanarGraph<W>, s: usize, t: usize, (du, dv): (usize,usize)) -> (W, Vec<PlanarEdge<W>>) {
    if let Some(p) = bfs(graph, s, t, (du,dv)) {
        let path = p.iter()
            .map(|e| e.rotate_right())
            .collect();
        let diversion = graph.N(du).iter()
            .find(|l| l.to == dv)
            .expect("The diversion edge doesn't exist")
            .clone();
        let (split, map) = split_edges(graph.dual(), path);
        match shortest_odd_path(&split, diversion.left, diversion.right) {
            Impossible => {
                panic!("Uhhhhh there really should be an odd path here, but we couldn't find it");
            }
            Possible {cost, path} => {
                let mapped: Vec<PlanarEdge<W>> = path.iter().flat_map(|e| map(e)).collect();
                let rotated: Vec<PlanarEdge<W>> = mapped.iter().map(|e| e.rotate_right()).collect();
                println!("Dual diversion set: {:?}", mapped);
                println!("Real diversion set: {:?}\n", rotated);

                (
                    cost,
                    rotated,
                )
            }
        }
    }
    else {
        println!("Could not find any s-t-path that doesn't use the diversion edge, no diversion is needed.");
        return (0.into(), Vec::new());
    }
}

fn bfs<W: Weight>(graph: &PlanarGraph<W>, s: usize, t: usize, (du,dv): (usize, usize)) -> Option<Vec<PlanarEdge<W>>> {
    let mut seen = repeat(graph.n(), false);
    let mut prev: Vec<Option<PlanarEdge<W>>> = repeat(graph.n(), None);
    let mut q: Queue<usize> = Queue::new();
    seen[s] = true;
    q.add(s).ok()?;

    while let Ok(u) = q.remove() {
        for line in graph.N(u) {
            let v = line.to;
            if (u,v) != (du,dv) && (v,u) != (du,dv) && ! seen[v] {
                seen[v] = true;
                q.add(v).ok()?;
                prev[v] = Some(line.clone());
                if v == t {
                    break;
                }
            }
        }
    }

    if seen[t] {
        let mut ret: Vec<PlanarEdge<W>> = vec![prev[t].clone().unwrap()];
        let mut curr = ret[0].clone();
        while curr.from != s {
            curr = prev[curr.from].clone().unwrap();
            ret.push(curr.clone());
        }
        return Some(ret);
    }
    None
}