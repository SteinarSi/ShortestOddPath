use std::fmt::{Debug, Display};
use std::str::FromStr;
use shortest_odd_path::algorithm::network_diversion::network_diversion;
use shortest_odd_path::algorithm::shortest_path::bfs;
use shortest_odd_path::structure::cost::{Cost, Cost::*};
use shortest_odd_path::structure::graph::planar_edge::PlanarEdge;
use shortest_odd_path::structure::graph::planar_graph::PlanarGraph;
use shortest_odd_path::structure::weight::Weight;
use utility::Problem;

mod utility;

pub struct NetworkDiversion;

impl <W> Problem<W> for NetworkDiversion
    where W: Weight,
          <W as FromStr>::Err: Debug + Display,
{
    type Output = Option<(W, Vec<PlanarEdge<W>>)>;
    type Query = (usize,usize,(usize,usize));
    type Expected = Cost<W>;
    type GraphClass = PlanarGraph<W>;

    fn name() -> String {
        "diversion".to_string()
    }

    fn parse_query(query: &str) -> Option<(Self::Query, Option<Self::Expected>)> {
        let mut words = query.split(' ');
        Some(((words.next()?.parse().ok()?,
               words.next()?.parse().ok()?,
              (words.next()?.parse().ok()?, words.next()?.parse().ok()?),
            ),
              words.next().map(|w| Cost::from(w.parse()))
        ))
    }

    fn verify_answer(planar: &Self::GraphClass, &(s,t,(du,dv)): &Self::Query, expected: &Option<Self::Expected>, out: &Self::Output) {
        let context = format!("Network Diversion from {} to {}, every path must go through ({},{}):", s,t,du,dv);
        if let Some(exp) = expected {
            match (exp, out) {
                (Infinite, Some(_)) => panic!("{}\nNo diversion is supposed to be possible here, but we found one anyway?", context),
                (Finite(_), None) => panic!("{}\nWe could not find any diversions, even though it should be possible", context),
                (Finite(exp_cost), Some((actual_cost, _))) => {
                    assert_eq!(exp_cost, actual_cost, "{}\nThe expected and actual costs do not match: {} != {}", context, exp_cost, actual_cost);
                }
                _ => {},
            }
        }
        if let Some((_, diversion)) = out {
            let mut g = planar.real().clone();
            let mut bottleneck = g.find_edges(du, dv);

            let connected = bfs(&g, s)[t].is_finite();
            assert!(connected, "{}\nNo {}-{}-paths exists, but we found a diversion anyway.", context, s, t);

            g.delete_edges(diversion);
            let non_blocking = bfs(&g, s)[t].is_finite();
            assert!(non_blocking, "{}\nThe diversion set is not minimal, and blocks off all {}-{}-paths even without the diversion edge.", context, s, t);

            bottleneck.extend(diversion.clone());
            g.delete_edges(&bottleneck);
            let is_bridge = bfs(&g, s)[t].is_infinite();
            assert!(is_bridge, "{}\nThe diversion edge is not a bridge, removing it does not block all {}-{}-paths", context, s, t);
        }
    }

    fn compute(graph: &Self::GraphClass, &(s,t,(u,v)): &Self::Query) -> Self::Output {
        network_diversion(graph, s, t,(u,v))
    }
}

#[cfg(test)]
mod test_network_diversion {
    mod small_planar_diversions {
        use crate::NetworkDiversion;
        use crate::utility::meta_test;

        fn test_diversion(name: &str) {
            meta_test::<NetworkDiversion, f64>("planar_graphs/small_planar_graphs", name);
        }

        #[test]
        fn small_planar1() { test_diversion("small_planar1"); }

        #[test]
        fn small_planar2() { test_diversion("small_planar2"); }

        #[test]
        fn small_planar3() { test_diversion("small_planar3"); }

        #[test]
        fn small_planar4() { test_diversion("small_planar4"); }

        #[test]
        fn small_planar5() { test_diversion("small_planar5"); }
    }
    
    mod delaunay_diversions {
        use crate::NetworkDiversion;
        use crate::utility::meta_test;

        fn test_delaunay(i: usize) {
            meta_test::<NetworkDiversion, f64>(
                "delaunay_graphs/planar_delaunay_graphs",
                ["delaunay", &i.to_string()].concat().as_str()
            );
        }

        #[test]
        fn all_delaunay_diversions() {
            (5..=100)
                .step_by(5)
                .for_each(test_delaunay)
        }
        
        #[test]
        fn delaunay50() {
            test_delaunay(50);
        }
    }
}
