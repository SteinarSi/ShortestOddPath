use criterion::{BenchmarkGroup, BenchmarkId, Criterion};
use criterion::measurement::WallTime;
use shortest_odd_path::structure::graph::edge::BasicEdge;
use shortest_odd_path::structure::graph::undirected_graph::UndirectedGraph;
use shortest_odd_path::structure::path_result::PathResult;

pub fn bench_trips(c: &mut Criterion, groupname: &str, alg: fn(&UndirectedGraph<u64, BasicEdge<u64>>, usize, usize) -> PathResult<u64, BasicEdge<u64>>) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group(groupname);
    group.sample_size(20);
    let inputs = [
        // "power-494-bus.mtx",           // n = 495
        // "power-1138-bus.mtx",          // n = 1138
        // "power-bcspwr09.mtx",          // n = 1723
        // "web-EPA.mtx",                 // n = 4271
        // "power-bcspwr10.mtx",          // n = 5300
        "CityOfOldenburg.in",          // n = 6105, m = 7035
        // "fb-pages-government.mtx",     // n = 7057
        // "twitch.in",                   // n = 7126
        // "COX2-MD.mtx",                 // n = 7963
        "CityOfSanJoaquinCounty.in",   // n = 18263
        // "COX2.mtx",                    // n = 19239
        "CaliforniaRoadNetwork.in",    // n = 21048, m = 21693
        "musae-github.in",             // n = 37700
        "SanFranciscoRoadNetwork.in",  // n = 174956, m = 223001
        // "NorthAmericaRoadNetwork.in",  // n = 175813
        "soc-pokec-relationships.in",  // n = 1632804, m = 30622565
    ];
    for input in inputs {
        let (graph, s, t) = read_input(&["data/real_graphs/", input].concat());
        group.bench_function(BenchmarkId::from_parameter(input), |b| {
            b.iter(|| alg(&graph, s, t));
        });
    }
}

pub fn read_input(path: &str) -> (UndirectedGraph<u64, BasicEdge<u64>>, usize, usize) {
    let input = std::fs::read_to_string(path).expect(format!("Could not read the graph '{}'.", path).as_str());
    let graph = UndirectedGraph::from(input);
    let t = graph.n()-1;
    (graph, 0, t)
}
