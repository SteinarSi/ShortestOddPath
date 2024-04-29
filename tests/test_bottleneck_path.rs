use std::fmt::{Debug, Display};
use std::str::FromStr;
use shortest_odd_path::algorithm::bottleneck_path::shortest_bottleneck_path;
use shortest_odd_path::structure::cost::{Cost, Finite, Infinite};
use shortest_odd_path::structure::graph::edge::{BasicEdge, Edge};
use shortest_odd_path::structure::graph::undirected_graph::UndirectedGraph;
use shortest_odd_path::structure::path_result::PathResult;
use shortest_odd_path::structure::path_result::PathResult::{Impossible, Possible};
use shortest_odd_path::structure::weight::Weight;
use utility::{Problem, verify_path};
use crate::utility::meta_test;

mod utility;

pub struct ShortestBottleneckPath;
impl <W> Problem<W> for ShortestBottleneckPath
    where W: Weight,
          <W as FromStr>::Err: Debug + Display,
{
    type Output = PathResult<W,BasicEdge<W>>;
    type Query = (usize, usize, (usize,usize), Cost<W>);
    type GraphClass = UndirectedGraph<W,BasicEdge<W>>;
    fn name() -> String { String::from("bottleneck") }
    fn parse_query(query: &str) -> Option<Self::Query> {
        let mut words = query.split(' ');
        Some((
            words.next()?.parse().ok()?,
            words.next()?.parse().ok()?,
            (words.next()?.parse().ok()?, words.next()?.parse().ok()?),
            Cost::from(words.next()?.parse())
        ))
    }
    fn display_query((s,t, (u,v), _): &Self::Query) -> String {
        format!("Bottlenecked path from {} to {}, passing through ({},{}):", s, t, u, v)
    }

    fn verify_answer(graph: &Self::GraphClass, query: &Self::Query, actual: &Self::Output) {
        let (source,sink, (u,v), expected_cost) = query;
        let context = Self::display_query(query);
        match (expected_cost, actual) {
            (Infinite, Possible{cost: _, path: _}) => panic!("{}\nExpected not to find a path, but the alg did anyway!", context),
            (Finite(c), Impossible) => panic!("{}\nExpected a path of cost {}, but the alg couldn't find it!", context, c),
            (Finite(expected_cost), Possible {cost: actual_cost, path}) => {
                verify_path::<W,BasicEdge<W>,Self::GraphClass,Self>(graph, &context, *expected_cost, *actual_cost, path, *source, *sink);
                assert!(path.iter().find(|e| e.from() == *u && e.to() == *v).is_some(), "{}\nThe path was supposed to go through the bottleneck of ({},{}), but it doesn't.", context, u, v);
            },
            _ => {},
        }
    }

    fn compute(graph: &Self::GraphClass, (source, sink, (u,v), _): &Self::Query) -> Self::Output {
        shortest_bottleneck_path::<W,BasicEdge<W>>(graph, *source, *sink, (*u,*v))
    }
}

fn test(folder: &str, name: &str) {
    meta_test::<ShortestBottleneckPath, u64>(folder, name);
}

#[test]
fn medium1() { test("medium_graphs", "medium1"); }