use criterion::{BenchmarkGroup, BenchmarkId, Criterion};
use criterion::measurement::WallTime;
use shortest_odd_path::structure::path_result::PathResult;
use shortest_odd_path::structure::graph::Graph;
use shortest_odd_path::structure::undirected_graph::UndirectedGraph;

pub fn bench_trips(c: &mut Criterion, groupname: &str, alg: fn(&UndirectedGraph, usize, usize) -> PathResult) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group(groupname);
    group.sample_size(10);
    let inputs = [
        "power-494-bus.mtx",       // n = 495
        "power-1138-bus.mtx",      // n = 1138
        "power-bcspwr09.mtx",      // n = 1723
        "web-EPA.mtx",                 // n = 4271
        "power-bcspwr10.mtx",      // n = 5300
        // "fb-pages-government.mtx", // n = 7057
        "COX2-MD.mtx",             // n = 7963
        "COX2.mtx",                // n = 19239
    ];
    for input in inputs {
        let (graph, s, t) = read_input(&["data/real_graphs/", input].concat());
        group.bench_function(BenchmarkId::from_parameter(input), |b| {
            b.iter(|| alg(&graph, s, t));
        });
    }
}

pub fn read_input(path: &str) -> (UndirectedGraph, usize, usize) {
    let input = std::fs::read_to_string(path).expect(format!("Could not read the graph '{}'.", path).as_str());
    let graph = UndirectedGraph::from(input);
    let t = graph.n()-1;
    (graph, 0, t)
}
