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
