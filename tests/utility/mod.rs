use std::fmt::{Debug, Display};
use std::str::FromStr;
use shortest_odd_path::structure::graph::edge::Edge;
use shortest_odd_path::structure::graph::graph::Graph;
use shortest_odd_path::structure::weight::Weight;

pub fn meta_test<Pr, W: Weight>(folder: &str, name: &str)
    where Pr: Problem<W>,
          <Pr as Problem<W>>::GraphClass: FromStr + Debug,
          <<Pr as Problem<W>>::GraphClass as FromStr>::Err: Debug,
          W: Weight,
          <W as FromStr>::Err: Display + Debug,
{
    let input_path = ["data/", folder, "/", name, "/", name, ".in"].concat();
    let query_path = ["data/", folder, "/", name, "/", name, ".", &Pr::name()].concat();
    let queries: Vec<Pr::Query> = std::fs::read_to_string(&query_path)
        .expect(&format!("Could not find the queries: {}", query_path))
        .lines()
        .map(|line| Pr::parse_query(line).expect("Could not parse query :-("))
        .collect();
    let graph = std::fs::read_to_string(&input_path)
        .expect(&format!("Could not find graph: {}", input_path))
        .parse()
        .expect("Could not parse the graph");
    for query in queries {
        Pr::verify_answer(&graph, &query, &Pr::compute(&graph, &query));
    }
    println!("Success :-)")
}

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

pub fn verify_path<'a, W, E, G, Pr>(graph: &G, context: &String, expected_cost: W, actual_cost: W, path: &Vec<E>, source: usize, sink: usize)
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
