use std::fmt::{Debug, Display};
use std::str::FromStr;
use shortest_odd_path::algorithm::odd_path::shortest_odd_path;
use shortest_odd_path::structure::cost::{Cost, Finite, Infinite};
use shortest_odd_path::structure::graph::edge::{BasicEdge, Edge};
use shortest_odd_path::structure::graph::undirected_graph::UndirectedGraph;
use shortest_odd_path::structure::path_result::PathResult;
use shortest_odd_path::structure::path_result::PathResult::{Impossible, Possible};
use shortest_odd_path::structure::weight::Weight;
use shortest_odd_path::utility::misc::debug;
use crate::utility::{meta_test, Problem, verify_path};

mod utility;

pub struct ShortestOddPath;
impl <W> Problem<W> for ShortestOddPath
    where W: Weight,
          <W as FromStr>::Err: Debug + Display,
{
    type Output = PathResult<W,BasicEdge<W>>;
    type Query = (usize, Cost<W>);
    type GraphClass = UndirectedGraph<W,BasicEdge<W>>;
    fn name() -> String { String::from("path") }
    fn parse_query(query: &str) -> Option<Self::Query> {
        let mut words = query.split(' ');
        let sink = words.next()?.parse().ok()?;
        let cost = words.next()?.parse().ok()?;
        Some((sink, cost))
    }
    fn display_query((t, _): &Self::Query) -> String {
        format!("Path from 0 to {}:", t)
    }
    fn verify_answer(graph: &Self::GraphClass, query: &Self::Query, actual: &Self::Output) {
        let (sink, expected) = query;
        let context = Self::display_query(query);
        match (expected, actual) {
            (Infinite, Possible {cost: _, path}) => panic!("{}\nExpected to not find any {}-{}-path, but found one anyway: {:?}", context, 0, sink, path),
            (Finite(cost), Impossible) => panic!("{}\nExpected the alg to find an {}-{}-path of cost {}, but it did not", context, 0, sink, cost),
            (Finite(expected_cost), Possible {cost: actual_cost, path}) => {
                assert_eq!(path.len() % 2, 1);
                verify_path::<W,BasicEdge<W>,Self::GraphClass,Self>(graph, &context, *expected_cost, *actual_cost, path, 0, *sink);
                for i in 0..path.len()-1 {
                    assert!(path[i+1..].iter().find(|e| e.to() == path[i].from()).is_none(), "{}\nThis was supposed to be a simple path, but {} was used at least twice!", context, path[i].from());
                }
            },
            _ => {}
        }
    }
    fn compute(graph: &Self::GraphClass, (sink, _): &Self::Query) -> Self::Output {
        shortest_odd_path(graph, 0, *sink)
    }
}

fn test_path(folder: &str, name: &str) {
    debug(format!("{}/{}", folder, name));
    meta_test::<ShortestOddPath, u64>(folder, name)
}
mod small_paths {
    use crate::test_path;

    fn test(name: &str) { test_path("small_graphs", name); }

    #[test]
    fn small1() { test("small1"); }
    #[test]
    fn small2() { test("small2"); }
    #[test]
    fn small3() { test("small3"); }
    #[test]
    fn small4() { test("small4"); }
    #[test]
    fn small5() { test("small5"); }
    #[test]
    fn small6() { test("small6"); }
    #[test]
    fn small7() { test("small7"); }
}

mod medium_paths {
    use crate::test_path;

    fn test(name: &str) { test_path("medium_graphs", name); }

    #[test]
    fn medium1() { test("medium1"); }
    #[test]
    fn medium2() { test("medium2"); }
    #[test]
    fn medium3() { test("medium3"); }
    #[test]
    fn medium4() { test("medium4"); }
    #[test]
    fn medium5() { test("medium5"); }
    #[test]
    fn medium6() { test("medium6"); }
}

mod large_paths {
    use crate::test_path;

    fn test(name: &str) { test_path("large_graphs", name); }

    #[test]
    fn large1() { test("large1"); }
    #[test]
    fn large2() { test("large2"); }
    #[test]
    fn large3() { test("large3"); }
}

mod special_paths {
    fn test(name: &str) { crate::test_path("special_graphs", name); }

    #[test]
    fn gamma4() { test("gamma4"); }

    #[test]
    fn petersen() { test("petersen"); }

    #[test]
    fn grid5() { test("grid5"); }
}
