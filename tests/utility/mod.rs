use crate::utility::problem::{Problem};
use shortest_odd_path::structure::undirected_graph::UndirectedGraph;

pub mod problem;

pub fn meta_test<Pr>(folder: &str, name: &str)
    where Pr: Problem,
{
    let queries: Vec<Pr::Query> = std::fs::read_to_string(format_answer_filepath::<Pr>(folder, name))
        .expect(&format!("Could not find the queries: {}", format_input_filepath(folder, name)))
        .lines()
        .map(|line| Pr::parse_query::<usize>(line).expect("Could not parse query :-("))
        .collect();
    let graph = UndirectedGraph::from(std::fs::read_to_string(&format_input_filepath(folder, name))
        .expect(&format!("Could not find graph: {}", format_input_filepath(folder, name))));
    for query in queries {
        Pr::verify_answer(&query, &Pr::compute(&graph, &query));
    }
    println!("Success :-)")
}

fn format_input_filepath(folder: &str, name: &str) -> String {
    ["data/", folder, "/", name, "/", name, ".in"].concat()
}

fn format_answer_filepath<Pr: Problem>(folder: &str, name: &str) -> String {
    ["data/", folder, "/", name, "/", name, ".", &Pr::name()].concat()
}
