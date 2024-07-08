use std::fs::read_to_string;

pub mod structure;
pub mod utility;
pub mod algorithm;

use shortest_odd_path::structure::graph::planar_graph::PlanarGraph;
use shortest_odd_path::algorithm::network_diversion::network_diversion;

use std::env;
use std::time::Instant;


fn parse_graph(filename: &str) -> PlanarGraph<f64> {
    read_to_string(filename)
        .expect("Could not find the graph")
        .parse()
        .expect("Could not read the graph")
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 6 {
        eprintln!("Usage: {} <file_name> <int1> <int2> <int3> <int4>",
                  args[0]);
        std::process::exit(1);
    }

    let fname = &args[1];
    let s: usize = args[2].parse().expect("s must be an integer");
    let t: usize = args[3].parse().expect("t must be an integer");
    let b1: usize = args[4].parse().expect("b1 must be an integer");
    let b2: usize = args[5].parse().expect("b2 must be an integer");

    println!("File name: {}", fname);
    println!("s = {}, t = {}, b = ({}, {})", s,t,b1,b2);

    let graph = parse_graph(fname);

    let start_time = Instant::now();


    if let Some((W, v)) = network_diversion(&graph, s, t, (b1,b2)) {
        println!("{:.2}", W);
    } else {
        println!("No found");
    }
    let duration = start_time.elapsed();
    println!("Time taken: {} ms", duration.as_millis());
}
