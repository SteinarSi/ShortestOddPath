use std::fmt::{Debug, Display};
use std::str::FromStr;
use shortest_odd_path::algorithm::bottleneck_path::shortest_bottleneck_path;
use shortest_odd_path::algorithm::network_diversion::network_diversion;
use shortest_odd_path::algorithm::odd_path::shortest_odd_path;
use shortest_odd_path::algorithm::odd_walk::shortest_odd_walk;
use shortest_odd_path::algorithm::shortest_path::bfs;
use shortest_odd_path::algorithm::two_disjoint_paths::two_disjoint_paths;
use shortest_odd_path::structure::cost::{Cost, Finite, Infinite};
use shortest_odd_path::structure::graph::edge::{BasicEdge, Edge};
use shortest_odd_path::structure::graph::graph::Graph;
use shortest_odd_path::structure::graph::planar::planar_edge::PlanarEdge;
use shortest_odd_path::structure::path_result::{PathResult, PathResult::*};
use shortest_odd_path::structure::graph::planar::planar_graph::PlanarGraph;
use shortest_odd_path::structure::graph::undirected_graph::UndirectedGraph;
use shortest_odd_path::structure::weight::Weight;

pub trait Problem<W>
    where W: Weight,
          <W as FromStr>::Err: Debug + Display,
{
    type Output;
    type Query;
    type GraphClass;
    fn name() -> String;
    fn parse_query(query: &str) -> Option<Self::Query>;
    fn display_query(query: &Self::Query) -> String;
    fn verify_answer(graph: &Self::GraphClass, expected: &Self::Query, actual: &Self::Output);
    fn compute(graph: &Self::GraphClass, query: &Self::Query) -> Self::Output;
}

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
                verify_path::<W,BasicEdge<W>,Self::GraphClass,Self>(graph, &context, *expected_cost, *actual_cost, path, 0, *sink);
            },
            _ => {}
        }
    }
    fn compute(graph: &Self::GraphClass, (sink, _): &Self::Query) -> Self::Output {
        shortest_odd_walk(graph, 0, *sink)
    }
}

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

pub struct TwoDisjointPaths;
impl <W> Problem<W> for TwoDisjointPaths
    where W: Weight,
          <W as FromStr>::Err: Debug + Display,
{
    type Output = Option<(W, Vec<BasicEdge<W>>, Vec<BasicEdge<W>>)>;
    type Query = ((usize, usize), (usize, usize), Cost<W>);
    type GraphClass = UndirectedGraph<W,BasicEdge<W>>;
    fn name() -> String { String::from("disjoint") }
    fn parse_query(query: &str) -> Option<Self::Query> {
        let mut words = query.split(' ');
        Some((
             (words.next()?.parse().ok()?, words.next()?.parse().ok()?),
             (words.next()?.parse().ok()?, words.next()?.parse().ok()?),
            Cost::from(words.next()?.parse())
        ))
    }
    fn display_query(((s1,t1), (s2,t2), _): &Self::Query) -> String {
        format!("Disjoint path from {} to {}, and from {} to {}:", s1, t1, s2, t2)
    }
    fn verify_answer(graph: &Self::GraphClass, query: &Self::Query, actual: &Self::Output) {
        let ((s1, t1), (s2,t2), cost) = query;
        let context = Self::display_query(query);
        match (cost, actual) {
            (Finite(_), None) => panic!("{}\nCould not find two vertex-disjoint paths, but it *should* be possible!", context),
            (Infinite, Some((_, p1, p2))) => panic!("{}\nWe didn't expect to find two vertex-disjoint paths from {} to {} and from {} to {}, but we did anyway: \n{:?}\nand\n{:?} ",context, s1, t1, s2, t2, p1,p2),
            (Finite(c), Some((w, p1,p2))) => {
                assert!( ! p1.iter().any(|u| p2.contains(&u)), "\n{}The two paths were supposed to use different vertices, but they don't:\n{:?}\nand\n{:?}", context, p1,p2);
                assert_eq!(*c, *w, "{}\nExpected two paths from {} to {} and from {} to {} with a combined length of {}, but found two of length {} instead!\n\n{:?}\nand\n{:?}", context, s1, t1, s2, t2, c, w, p1, p2);
                verify_path::<W,BasicEdge<W>,Self::GraphClass,Self>(&graph, &context, 0.into(), 0.into(), p1, *s1, *t1);
                verify_path::<W,BasicEdge<W>,Self::GraphClass,Self>(&graph, &context, 0.into(), 0.into(), p2, *s2, *t2);
            }
            _ => {}
        }
    }
    fn compute(graph: &Self::GraphClass, (p1, p2, _): &Self::Query) -> Self::Output {
        two_disjoint_paths(graph, *p1, *p2)
    }
}

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
        format!("Network Diversion from {} to {}, every path must go through ({},{})", s,t,u,v)
    }

    fn verify_answer(graph: &Self::GraphClass, &(s,t,(du,dv),expected): &Self::Query, (cost, diversion): &Self::Output) {
        assert_eq!(expected, *cost);
        let mut bottleneck = graph.find_edges(du, dv);
        let dist_before = bfs(&graph.delete_edges(diversion), s);
        assert!(dist_before[t].is_finite());
        bottleneck.extend(diversion.clone());
        let dist_after = bfs(&graph.delete_edges(&bottleneck), s);
        assert!(dist_after[t].is_infinite());
    }

    fn compute(graph: &Self::GraphClass, &(s,t,(u,v),_): &Self::Query) -> Self::Output {
        network_diversion(graph, s, t,(u,v))
    }
}

fn verify_path<'a, W, E, G, Pr>(graph: &G, context: &String, expected_cost: W, actual_cost: W, path: &Vec<E>, source: usize, sink: usize)
    where W: Weight,
          <W as FromStr>::Err: Debug + Display,
          E: Edge<W>,
          G: Graph<'a, E, W>,
          Pr: Problem<W>,
{
    assert_eq!(expected_cost, actual_cost, "{}\nThe costs don't match: expected {}, but got {}.\nThe offending path: {:?}", context, expected_cost, actual_cost, path);
    assert_eq!(source, path[0].from(), "{}\nThe path starts at the wrong vertex! Expected {}, but yet it starts at {} for some reason", context, source, path[0].from());
    assert_eq!(sink, path[path.len()-1].to(), "{}\nThe path ends at the wrong vertex! Expected {}, but it ends at {} for some strange reason that you should consider debugging.", context, sink, path[path.len()-1].to());
    for e in path {
        assert!(graph.is_adjacent(e.from(), e.to()), "{}\nOur path includes an edge from {} to {} that doesn't exist in the graph!", context, e.from(), e.to())
    }
}