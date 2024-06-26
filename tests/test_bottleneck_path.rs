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
    type Query = (usize, usize, (usize,usize));
    type Expected = Cost<W>;
    type GraphClass = UndirectedGraph<W,BasicEdge<W>>;
    fn name() -> String { String::from("bottleneck") }
    fn parse_query(query: &str) -> Option<(Self::Query, Option<Self::Expected>)> {
        let mut words = query.split(' ');
        Some(((words.next()?.parse().ok()?,
               words.next()?.parse().ok()?,
              (words.next()?.parse().ok()?, words.next()?.parse().ok()?),
            ), 
            words.next().map(|w| Cost::from(w.parse()))
        ))
    }

    fn verify_answer(graph: &Self::GraphClass, query: &Self::Query, expected: &Option<Self::Expected>, actual: &Self::Output) {
        let (source,sink, (u,v)) = query;
        let context = format!("Bottlenecked path from {} to {}, passing through ({},{}):", source, sink, u, v);
        if let Some(exp) = expected {
            match (exp, actual) {
                (Infinite, Possible{cost: _, path: _}) => panic!("{}\nExpected not to find a path, but the alg did anyway!", context),
                (Finite(c), Impossible) => panic!("{}\nExpected a path of cost {}, but the alg couldn't find it!", context, c),
                (Finite(expected_cost), Possible {cost: actual_cost, path}) => {
                    assert_eq!(expected_cost, actual_cost, "{}\nThe costs don't match: expected {}, but got {}.\nThe offending path: {:?}", context, expected_cost, actual_cost, path);
                },
                _ => {},
            }
        }
        if let Possible {cost, path} = actual {
            verify_path::<W,BasicEdge<W>,Self>(graph, &context, *cost, path, *source, *sink);
            assert!(path.iter().find(|e| e.from() == *u && e.to() == *v).is_some(), "{}\nThe path was supposed to go through the bottleneck of ({},{}), but it doesn't.", context, u, v);
        }
    }

    fn compute(graph: &Self::GraphClass, (source, sink, (u,v)): &Self::Query) -> Self::Output {
        shortest_bottleneck_path::<W,BasicEdge<W>>(graph, *source, *sink, (*u,*v))
    }
}

fn test(folder: &str, name: &str) {
    meta_test::<ShortestBottleneckPath, u64>(folder, name);
}

#[test]
fn medium1() { test("medium_graphs", "medium1"); }