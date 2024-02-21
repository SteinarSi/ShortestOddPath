use crate::structure::cost::{Cost, Cost::*};

pub fn format_costs(dist: &Vec<Cost>) -> String {
    (0..dist.len())
        .map(|u| format!("dist({}) = {}\n", u, format_cost(dist[u])))
        .collect::<Vec<_>>()
        .concat()
}

pub fn format_cost(cost: Cost) -> String {
    match cost {
        Finite(x) => x.to_string(),
        Infinite => String::from("∞"),
    }
}

pub fn format_input_filepath(folder: &str, name: &str) -> String {
    ["data/", folder, "/", name, "/", name, ".in"].concat()
}

pub fn format_answer_filepath(folder: &str, name: &str, problem: &str) -> String {
    ["data/", folder, "/", name, "/", name, ".", problem].concat()
}