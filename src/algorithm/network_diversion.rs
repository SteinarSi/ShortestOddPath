use queues::{IsQueue, Queue};
use crate::algorithm::odd_path::shortest_odd_path;
use crate::algorithm::utility::split_edges;
use crate::structure::graph::graph::Graph;
use crate::structure::path_result::{PathResult::*};
use crate::structure::graph::planar::planar_edge::{PlanarEdge, PlanarEdgeImpl};
use crate::structure::graph::planar::planar_graph::PlanarGraph;
use crate::structure::weight::Weight;
use crate::utility::misc::{debug, repeat};

pub fn network_diversion<W: Weight>(graph: &PlanarGraph<W>, s: usize, t: usize, (du, dv): (usize,usize)) -> (W, Vec<PlanarEdgeImpl<W>>) {
    let path: Vec<PlanarEdgeImpl<W>> = bfs(graph, s, t, (du, dv))
        .expect("Could not find an s-t-path at all, the graph isn't connected")
        .iter()
        .map(|e| e.rotate_right())
        .collect();
    debug(format!("Path: {:?}", path));
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
            let mapped: Vec<PlanarEdgeImpl<W>> = path.iter().flat_map(|e| map(e)).collect();
            let rotated: Vec<PlanarEdgeImpl<W>> = mapped.iter().map(|e| e.rotate_right()).collect();
            debug(format!("\nDual diversion set: {:?}", mapped));
            debug(format!("Real diversion set: {:?}\n", rotated));

            (
                cost,
                rotated,
            )
        }
    }
}

fn bfs<W: Weight>(graph: &PlanarGraph<W>, s: usize, t: usize, (du,dv): (usize, usize)) -> Option<Vec<PlanarEdgeImpl<W>>> {
    let mut seen = repeat(graph.n(), false);
    let mut prev: Vec<Option<PlanarEdgeImpl<W>>> = repeat(graph.n(), None);
    let mut q: Queue<usize> = Queue::new();
    seen[s] = true;
    q.add(s).ok()?;

    while let Ok(u) = q.remove() {
        for line in graph.N(u) {
            let v = line.to;
            if (u,v) != (du,dv) && ! seen[v] {
                seen[v] = true;
                q.add(v).ok()?;
                prev[v] = Some(line);
                if v == t {
                    break;
                }
            }
        }
    }

    if seen[t] {
        let mut ret: Vec<PlanarEdgeImpl<W>> = vec![prev[t].clone().unwrap()];
        let mut curr = ret[0].clone();
        while curr.from != s {
            curr = prev[curr.from].clone().unwrap();
            ret.push(curr.clone());
        }
        // TODO reversing isn't necessary, this is just for debugging purposes
        ret.reverse();
        return Some(ret);
    }
    None
}