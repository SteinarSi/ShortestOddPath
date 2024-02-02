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

pub fn format_input_filepath(folder: &str, file: &str) -> String {
    ["data/", folder, "/", file, ".in"].concat()
}

pub fn format_answer_filepath(folder: &str, file: &str, problem: &str) -> String {
    ["data/", folder, "/", file, "_", problem, ".ans"].concat()
}