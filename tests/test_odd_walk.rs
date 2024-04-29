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
    type Query = (usize, Cost<W>);
    type GraphClass = UndirectedGraph<W,BasicEdge<W>>;
    fn name() -> String {
        String::from("walk")
    }
    fn parse_query(query: &str) -> Option<Self::Query> {
        let mut words = query.split(' ');
        let sink = words.next()?.parse().ok()?;
        let cost = words.next()?.parse().ok()?;
        Some((sink, cost))
    }
    fn display_query((t, _): &Self::Query) -> String {
        format!("Walk from 0 to {}:", t)
    }
    fn verify_answer(graph: &Self::GraphClass, query: &Self::Query, actual: &Self::Output) {
        let (sink, expected) = query;
        let context = Self::display_query(query);
        match (expected, actual) {
            (Infinite, Possible {cost: _, path}) => panic!("{}\nExpected to not find any {}-{}-walk, but found one anyway: {:?}", context, 0, sink, path),
            (Finite(cost), Impossible) => panic!("{}\nExpected the alg to find an {}-{}-walk of cost {}, but it did not", context, 0, sink, cost),
            (Finite(expected_cost), Possible {cost: actual_cost, path}) => {
                assert_eq!(path.len() % 2, 1);
                verify_path::<W, BasicEdge<W>, Self::GraphClass, Self>(graph, &context, *expected_cost, *actual_cost, path, 0, *sink);
            },
            _ => {}
        }
    }
    fn compute(graph: &Self::GraphClass, (sink, _): &Self::Query) -> Self::Output {
        shortest_odd_walk(graph, 0, *sink)
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