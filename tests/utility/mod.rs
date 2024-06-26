use std::fmt::{Debug, Display};
use std::str::FromStr;
use shortest_odd_path::structure::graph::edge::Edge;
use shortest_odd_path::structure::graph::undirected_graph::UndirectedGraph;
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
    let queries: Vec<(Pr::Query, Option<Pr::Expected>)> = std::fs::read_to_string(&query_path)
        .expect(&format!("Could not find the queries: {}", query_path))
        .lines()
        .map(|line| Pr::parse_query(line).expect(format!("Could not parse this query: {}", line).as_str()))
        .collect();
    let graph = std::fs::read_to_string(&input_path)
        .expect(&format!("Could not find graph: {}", input_path))
        .parse()
        .expect("Could not parse the graph");
    for (query, expected) in queries {
        Pr::verify_answer(&graph, &query, &expected, &Pr::compute(&graph, &query));
    }
    println!("Success :-)")
}

pub trait Problem<W>
    where W: Weight,
          <W as FromStr>::Err: Debug + Display,
{
    type Output;
    type Query;
    type Expected;
    type GraphClass;
    fn name() -> String;
    fn parse_query(query: &str) -> Option<(Self::Query, Option<Self::Expected>)>;
    fn verify_answer(graph: &Self::GraphClass, query: &Self::Query, expected: &Option<Self::Expected>, actual: &Self::Output);
    fn compute(graph: &Self::GraphClass, query: &Self::Query) -> Self::Output;
}

pub fn verify_path<'a, W, E, Pr>(graph: &UndirectedGraph<W,E>, context: &String, cost: W, path: &Vec<E>, source: usize, sink: usize)
    where W: Weight,
          <W as FromStr>::Err: Debug + Display,
          E: Edge<W>,
          Pr: Problem<W>,
{
    assert_eq!(source, path[0].from(), "{}\nThe path starts at the wrong vertex! Expected {}, but yet it starts at {} for some reason", context, source, path[0].from());
    assert_eq!(sink, path[path.len()-1].to(), "{}\nThe path ends at the wrong vertex! Expected {}, but it ends at {} for some strange reason that you should consider debugging.", context, sink, path[path.len()-1].to());
    let mut actual_cost = 0.into();
    for e in path {
        assert!(graph.is_adjacent(e.from(), e.to()), "{}\nOur path includes an edge from {} to {} that doesn't exist in the graph!", context, e.from(), e.to());
        actual_cost = actual_cost + e.weight();
    }
    assert_eq!(cost, actual_cost, "The path does not cost what it says it does: {} != {}", cost, actual_cost);
}
