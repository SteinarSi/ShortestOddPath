use std::fmt::{Debug, Display};
use std::str::FromStr;
use shortest_odd_path::structure::weight::Weight;
use crate::utility::problem::Problem;

pub mod problem;

pub fn meta_test<Pr, W: Weight>(folder: &str, name: &str)
    where Pr: Problem<W>,
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
