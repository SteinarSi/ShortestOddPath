use std::str::FromStr;
use shortest_odd_path::algorithm::bottleneck_path::shortest_bottleneck_path;
use shortest_odd_path::algorithm::odd_path::shortest_odd_path;
use shortest_odd_path::algorithm::odd_walk::shortest_odd_walk;
use shortest_odd_path::algorithm::two_disjoint_paths::two_disjoint_paths;
use shortest_odd_path::structure::cost::{Cost, Finite, Infinite};
use shortest_odd_path::structure::path_result::{PathResult, PathResult::*};
use shortest_odd_path::structure::undirected_graph::UndirectedGraph;

pub trait Problem{
    type Output;
    type Query;
    fn name() -> String;
    fn parse_query<F: FromStr>(query: &str) -> Option<Self::Query>;
    fn verify_answer(graph: &UndirectedGraph, expected: &Self::Query, actual: &Self::Output);
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
    fn verify_answer(graph: &UndirectedGraph, (sink, expected): &Self::Query, actual: &Self::Output) {
        match (expected, actual) {
            (Infinite, Possible {cost: _, path}) => panic!("Expected to not find any {}-{}-walk, but found one anyway: {:?}", 0, sink, path),
            (Finite(cost), Impossible) => panic!("Expected the alg to find an {}-{}-walk of cost {}, but it did not", 0, sink, cost),
            (Finite(expected_cost), Possible {cost: actual_cost, path}) => {
                verify_path(graph, *expected_cost, *actual_cost, path, 0, *sink);
            },
            _ => {}
        }
    }
    fn compute(graph: &UndirectedGraph, (sink, _): &Self::Query) -> Self::Output {
        shortest_odd_walk(graph, 0, *sink)
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
    fn verify_answer(graph: &UndirectedGraph, (sink, expected): &Self::Query, actual: &Self::Output) {
        match (expected, actual) {
            (Infinite, Possible {cost: _, path}) => panic!("Expected to not find any {}-{}-path, but found one anyway: {:?}", 0, sink, path),
            (Finite(cost), Impossible) => panic!("Expected the alg to find an {}-{}-path of cost {}, but it did not", 0, sink, cost),
            (Finite(expected_cost), Possible {cost: actual_cost, path}) => {
                verify_path(graph, *expected_cost, *actual_cost, path, 0, *sink);
                for i in 0..path.len()-1 {
                    assert!( ! path[i+1..].contains(&path[i]), "This was supposed to be a simple path, but {} was used at least twice!", path[i]);
                }
            },
            _ => {}
        }
    }
    fn compute(graph: &UndirectedGraph, (sink, _): &Self::Query) -> Self::Output {
        shortest_odd_path(graph, 0, *sink)
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

    fn verify_answer(graph: &UndirectedGraph, (source,sink, (u,v), expected_cost): &Self::Query, actual: &Self::Output) {
        match (expected_cost, actual) {
            (Infinite, Possible{cost: _, path: _}) => panic!("Expected not to find a path, but the alg did anyway!"),
            (Finite(c), Impossible) => panic!("Expected a path of cost {}, but the alg couldn't find it!", c),
            (Finite(expected_cost), Possible {cost: actual_cost, path}) => {
                verify_path(graph, *expected_cost, *actual_cost, path, *source, *sink);
                assert!((0..path.len()-1).find(|&i| (path[i], path[i+1]) == (*u,*v)).is_some(), "The path was supposed to go through the bottleneck of ({},{}), but it doesn't.", u, v);
            },
            _ => {},
        }
    }

    fn compute(graph: &UndirectedGraph, (source, sink, (u,v), _): &Self::Query) -> Self::Output {
        shortest_bottleneck_path(graph, *source, *sink, (*u,*v))
    }
}

pub struct TwoDisjointPaths;
impl Problem for TwoDisjointPaths {
    type Output = Option<(Vec<usize>, Vec<usize>)>;
    type Query = ((usize, usize), (usize, usize), Cost);
    fn name() -> String { String::from("disjoint") }
    fn parse_query<F: FromStr>(query: &str) -> Option<Self::Query> {
        let mut words = query.split(' ');
        Some((
             (words.next()?.parse().ok()?, words.next()?.parse().ok()?),
             (words.next()?.parse().ok()?, words.next()?.parse().ok()?),
            Cost::from(words.next()?.parse())
        ))
    }
    fn verify_answer(graph: &UndirectedGraph, ((s1, t1), (s2,t2), cost): &Self::Query, actual: &Self::Output) {
        match (cost, actual) {
            (Finite(_), None) => panic!("Could not find two vertex-disjoint paths, but it *should* be possible!"),
            (Infinite, Some((p1, p2))) => panic!("We didn't expect to find two vertex-disjoint paths from {} to {} and from {} to {}, but we did anyway: \n{:?}\nand\n{:?} ", s1, t1, s2, t2, p1,p2),
            (Finite(c), Some((p1,p2))) => {
                assert!( ! p1.iter().any(|u| p2.contains(&u)), "The two paths were supposed to use different vertices, but they don't:\n{:?}\nand\n{:?}", p1,p2);
                let cost = (p1.len() + p2.len() - 2) as u64;
                assert_eq!(*c, cost, "Expected two paths from {} to {} and from {} to {} with a combined lenght of {}, but found two of length {} instead!\n\n{:?}\nand\n{:?}", s1, t1, s2, t2, c, cost, p1, p2);
                verify_path(&graph, 0, 0, p1, *s1, *t1);
                verify_path(&graph, 0, 0, p2, *s2, *t2);
            }
            _ => {}
        }
    }
    fn compute(graph: &UndirectedGraph, (p1, p2, _): &Self::Query) -> Self::Output {
        two_disjoint_paths(graph, *p1, *p2)
    }
}

fn verify_path(graph: &UndirectedGraph, expected_cost: u64, actual_cost: u64, path: &Vec<usize>, source: usize, sink: usize) {
    assert_eq!(expected_cost, actual_cost, "The costs don't match: expected {}, but got {}", expected_cost, actual_cost);
    assert_eq!(source, path[0], "The path starts at the wrong vertex! Expected {}, but yet it starts at {} for some reason", source, path[0]);
    assert_eq!(sink, path[path.len()-1], "The path ends at the wrong vertex! Expected {}, but it ends at {} for some strange reason that you should consider debugging.", sink, path[path.len()-1]);
    for i in 0..path.len()-1 {
        let (u, v) = (path[i], path[i+1]);
        assert!(graph[u].contains(&v), "Our path includes an edge from {} to {} that doesn't exist in the graph!", u, v);
    }
}