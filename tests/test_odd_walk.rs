mod utility;

use std::fmt::{Debug, Display};
use std::str::FromStr;
use shortest_odd_path::algorithm::odd_walk::shortest_odd_walk;
use shortest_odd_path::structure::cost::{Cost, Finite, Infinite};
use shortest_odd_path::structure::graph::edge::BasicEdge;
use shortest_odd_path::structure::graph::undirected_graph::UndirectedGraph;
use shortest_odd_path::structure::path_result::PathResult;
use shortest_odd_path::structure::path_result::PathResult::{Impossible, Possible};
use shortest_odd_path::structure::weight::Weight;
use crate::utility::{meta_test, Problem, verify_path};

pub struct ShortestOddWalk;
impl <W> Problem<W> for ShortestOddWalk
    where W: Weight,
          <W as FromStr>::Err: Debug + Display,
{
    type Output = PathResult<W,BasicEdge<W>>;
    type Query = (usize, usize);
    type Expected = Cost<W>;
    type GraphClass = UndirectedGraph<W,BasicEdge<W>>;
    fn name() -> String {
        String::from("walk")
    }
    fn parse_query(query: &str) -> Option<(Self::Query, Option<Self::Expected>)> {
        let mut words = query.split(' ');
        let source = words.next()?.parse().ok()?;
        let sink = words.next()?.parse().ok()?;
        let cost = words.next()?.parse().ok();
        Some(((source, sink), cost))
    }

    fn verify_answer(graph: &Self::GraphClass, query: &Self::Query, expected: &Option<Self::Expected>, actual: &Self::Output) {
        let (source, sink) = query;
        let context = format!("Walk from {} to {}:", source, sink);
        if let Some(exp) = expected {
            match (exp, actual) {
                (Infinite, Possible { cost: _, path }) => panic!("{}\nExpected to not find any {}-{}-walk, but found one anyway: {:?}", context, source, sink, path),
                (Finite(cost), Impossible) => panic!("{}\nExpected the alg to find an {}-{}-walk of cost {}, but it did not", context, source, sink, cost),
                (Finite(expected_cost), Possible { cost: actual_cost, path }) => {
                    assert_eq!(expected_cost, actual_cost, "{}\nThe costs don't match: expected {}, but got {}.\nThe offending path: {:?}", context, expected_cost, actual_cost, path);
                },
                _ => {}
            }
        }
        if let Possible { cost, path } = actual {
            assert_eq!(path.len() % 2, 1);
            verify_path::<W, BasicEdge<W>, Self>(graph, &context, *cost, path, *source, *sink);
        }
    }
    fn compute(graph: &Self::GraphClass, (source, sink): &Self::Query) -> Self::Output {
        shortest_odd_walk(graph, *source, *sink)
    }
}

fn test_walk(folder: &str, file: &str) {
    meta_test::<ShortestOddWalk, u64>(folder, file)
}

mod small_walks {
    use crate::test_walk;

    fn test(name: &str) { test_walk("small_graphs", name); }

    #[test]
    fn small1() { test("small1"); }
    #[test]
    fn small2() { test("small2"); }
    #[test]
    fn small3() { test("small3"); }
    #[test]
    fn small4() { test("small4"); }
}

mod medium_walks {
    use crate::test_walk;

    fn test(name: &str) { test_walk("medium_graphs", name); }
    
    #[test]
    fn medium4() { test("medium4"); }
    #[test]
    fn medium5() { test("medium5"); }
}