use std::fmt::{Debug, Display};
use std::str::FromStr;
use shortest_odd_path::algorithm::network_diversion::network_diversion;
use shortest_odd_path::algorithm::shortest_path::bfs;
use shortest_odd_path::structure::graph::graph::Graph;
use shortest_odd_path::structure::graph::planar::planar_edge::PlanarEdge;
use shortest_odd_path::structure::graph::planar::planar_graph::PlanarGraph;
use shortest_odd_path::structure::weight::Weight;
use utility::Problem;

mod utility;

pub struct NetworkDiversion;

impl <W> Problem<W> for NetworkDiversion
    where W: Weight,
          <W as FromStr>::Err: Debug + Display,
{
    type Output = (W, Vec<PlanarEdge<W>>);
    type Query = (usize,usize,(usize,usize),W);
    type GraphClass = PlanarGraph<W>;

    fn name() -> String {
        "diversion".to_string()
    }

    fn parse_query(query: &str) -> Option<Self::Query> {
        let mut words = query.split(' ');
        Some((
            words.next()?.parse().ok()?,
            words.next()?.parse().ok()?,
            (words.next()?.parse().ok()?, words.next()?.parse().ok()?),
            words.next()?.parse().ok()?
        ))
    }

    fn display_query((s,t,(u,v),_): &Self::Query) -> String {
        format!("Network Diversion from {} to {}, every path must go through ({},{}):", s,t,u,v)
    }

    fn verify_answer(graph: &Self::GraphClass, &(s,t,(du,dv),expected): &Self::Query, (cost, diversion): &Self::Output) {
        assert_eq!(expected, *cost);
        let mut g = graph.clone();
        let mut bottleneck = g.find_edges(du, dv);
        g.delete_edges(diversion);
        let dist_before = bfs(&g, s);
        assert!(dist_before[t].is_finite());
        bottleneck.extend(diversion.clone());
        g.delete_edges(&bottleneck);
        let dist_after = bfs(&g, s);
        assert!(dist_after[t].is_infinite());
    }

    fn compute(graph: &Self::GraphClass, &(s,t,(u,v),_): &Self::Query) -> Self::Output {
        network_diversion(graph, s, t,(u,v))
    }
}

#[cfg(test)]
mod small_planar {
    use crate::NetworkDiversion;
    use crate::utility::meta_test;

    fn test_diversion(name: &str) {
        meta_test::<NetworkDiversion, f64>("planar_graphs", name);
    }

    #[test]
    fn small_planar1() { test_diversion("small_planar1"); }
    #[test]
    fn small_planar2() { test_diversion("small_planar2"); }
}
