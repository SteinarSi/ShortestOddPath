use criterion::{BenchmarkGroup, BenchmarkId, Criterion};
use criterion::measurement::WallTime;
use shortest_odd_path::structure::path_result::PathResult;
use shortest_odd_path::structure::graph::Graph;
use shortest_odd_path::structure::undirected_graph::UndirectedGraph;

pub fn bench_trips(c: &mut Criterion, groupname: &str, alg: fn(UndirectedGraph, usize, usize) -> PathResult) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group(groupname);
    group.sample_size(10);
    let inputs = [
        "power-494-bus.mtx",       // n = 495
        "fb-pages-government.mtx", // n = 7057
    ];
    for input in inputs {
        let (graph, s, t) = read_input(&["data/real_graphs/", input].concat());
        group.bench_function(BenchmarkId::from_parameter(input), |b| {
            b.iter(|| alg(graph.clone(), s, t));
        });
    }
}

pub fn read_input(path: &str) -> (UndirectedGraph, usize, usize) {
    let input = std::fs::read_to_string(path).expect("Could not read the graph :-(");
    let graph = UndirectedGraph::from(input);
    let t = graph.n()-1;
    (graph, 0, t)
}
