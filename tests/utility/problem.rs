use std::str::FromStr;
use shortest_odd_path::algorithm::bottleneck_path::shortest_bottleneck_path;
use shortest_odd_path::algorithm::odd_path::shortest_odd_path;
use shortest_odd_path::algorithm::odd_walk::shortest_odd_walk;
use shortest_odd_path::structure::cost::{Cost, Finite, Infinite};
use shortest_odd_path::structure::path_result::{PathResult, PathResult::*};
use shortest_odd_path::structure::undirected_graph::UndirectedGraph;

pub trait Problem{
    type Output;
    type Query;
    fn name() -> String;
    fn parse_query<F: FromStr>(query: &str) -> Option<Self::Query>;
    fn verify_answer(expected: &Self::Query, actual: &Self::Output);
    fn compute(graph: &UndirectedGraph, query: &Self::Query) -> Self::Output;
}

pub struct ShortestOddWalk;
impl Problem for ShortestOddWalk {
    type Output = PathResult;
    type Query = (usize, Cost);
    fn name() -> String {
        String::from("walk")
    }
    fn parse_query<F: FromStr>(query: &str) -> Option<Self::Query> {
        let mut words = query.split(' ');
        let sink = words.next()?.parse().ok()?;
        let cost = words.next()?.parse().ok()?;
        Some((sink, cost))
    }
    fn verify_answer((sink, expected): &Self::Query, actual: &Self::Output) {
        match (expected, actual) {
            (Infinite, Possible {cost: _, path}) => panic!("Expected to not find any {}-{}-walk, but found one anyway: {:?}", 0, sink, path),
            (Finite(cost), Impossible) => panic!("Expected the alg to find an {}-{}-walk of cost {}, but it did not", 0, sink, cost),
            (Finite(cost1), Possible {cost: cost2, path: _}) => assert_eq!(cost1, cost2, "Expected to find an {}-{}-walk of cost {}, but found one that costs {} instead", 0, sink, cost1, cost2),
            _ => {}
        }
    }
    fn compute(graph: &UndirectedGraph, (sink, _): &Self::Query) -> Self::Output {
        shortest_odd_walk(graph.clone(), 0, *sink)
    }
}

pub struct ShortestOddPath;
impl Problem for ShortestOddPath {
    type Output = PathResult;
    type Query = (usize, Cost);
    fn name() -> String { String::from("path") }
    fn parse_query<F: FromStr>(query: &str) -> Option<Self::Query> {
        let mut words = query.split(' ');
        let sink = words.next()?.parse().ok()?;
        let cost = words.next()?.parse().ok()?;
        Some((sink, cost))
    }
    fn verify_answer((sink, expected): &Self::Query, actual: &Self::Output) {
        match (expected, actual) {
            (Infinite, Possible {cost: _, path}) => panic!("Expected to not find any {}-{}-path, but found one anyway: {:?}", 0, sink, path),
            (Finite(cost), Impossible) => panic!("Expected the alg to find an {}-{}-path of cost {}, but it did not", 0, sink, cost),
            (Finite(cost1), Possible {cost: cost2, path: _}) => assert_eq!(cost1, cost2, "Expected to find an {}-{}-path of cost {}, but found one that costs {} instead", 0, sink, cost1, cost2),
            _ => {}
        }
    }
    fn compute(graph: &UndirectedGraph, (sink, _): &Self::Query) -> Self::Output {
        shortest_odd_path(graph.clone(), 0, *sink)
    }
}
pub struct ShortestBottleneckPath;
impl Problem for ShortestBottleneckPath {
    type Output = PathResult;
    type Query = (usize, usize, (usize,usize), Cost);
    fn name() -> String { String::from("bottleneck") }
    fn parse_query<F: FromStr>(query: &str) -> Option<Self::Query> {
        let mut words = query.split(' ');
        Some((
            words.next()?.parse().ok()?,
            words.next()?.parse().ok()?,
            (words.next()?.parse().ok()?, words.next()?.parse().ok()?),
            Cost::from(words.next()?.parse())
        ))
    }

    fn verify_answer((source,sink, (u,v), expected_cost): &Self::Query, actual: &Self::Output) {
        match (expected_cost, actual) {
            (Infinite, Possible{cost: _, path: _}) => panic!("Expected not to find a path, but the alg did anyway!"),
            (Finite(c), Impossible) => panic!("Expected a path of cost {}, but the alg couldn't find it!", c),
            (Finite(c), Possible{cost, path}) => {
                assert_eq!(c, cost, "The costs don't match: expected {}, but got {}", c, cost);
                assert_eq!(source, &path[0], "The path starts at the wrong vertex! Expected {}, but yet it starts at {} for some reason", source, path[0]);
                assert_eq!(sink, &path[path.len()-1], "The path ends at the wrong vertex! Expected {}, but it ends at {} for some strange reason that you should consider debugging.", sink, path[path.len()-1]);
                assert!((0..path.len()-1).find(|&i| (path[i], path[i+1]) == (*u,*v)).is_some(), "The path was supposed to go through the bottleneck of ({},{}), but it doesn't.", u, v);
            },
            _ => {},
        }
    }

    fn compute(graph: &UndirectedGraph, (source, sink, (u,v), _): &Self::Query) -> Self::Output {
        shortest_bottleneck_path(graph, *source, *sink, (*u,*v))
    }
}