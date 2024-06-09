use std::fs::read_to_string;
use criterion::{BenchmarkGroup, BenchmarkId, Criterion};
use criterion::measurement::WallTime;
use shortest_odd_path::structure::graph::edge::BasicEdge;
use shortest_odd_path::structure::graph::undirected_graph::UndirectedGraph;
use shortest_odd_path::structure::path_result::PathResult;
use shortest_odd_path::structure::weight::Weight;

pub fn bench_trip<W: Weight>(group: &mut BenchmarkGroup<WallTime>, folder: &str, filename: &str, alg: fn(&UndirectedGraph<W, BasicEdge<W>>, usize, usize) -> PathResult<W, BasicEdge<W>>) {
    let (graph, queries) = parse_bench(folder, filename);
    for (s, t) in queries {
        group.bench_function(BenchmarkId::from_parameter(filename), |b| {
            b.iter(|| alg(&graph, s, t))
        });
    }
}

pub fn bench_real_trips(c: &mut Criterion, groupname: &str, alg: fn(&UndirectedGraph<u64, BasicEdge<u64>>, usize, usize) -> PathResult<u64, BasicEdge<u64>>) {
    let trips = vec![
        "power-494-bus",           // n = 495
        // "power-1138-bus",          // n = 1138
        // "power-bcspwr09",          // n = 1723
        // "web-EPA",                 // n = 4271
        // "power-bcspwr10",          // n = 5300
        // "CityOfOldenburg",          // n = 6105, m = 7035
        // "fb-pages-government",     // n = 7057
        // "twitch",                   // n = 7126
        // "COX2-MD",                 // n = 7963
        // "CityOfSanJoaquinCounty",   // n = 18263
        // "COX2",                    // n = 19239
        // "CaliforniaRoadNetwork",    // n = 21048, m = 21693
        // "musae-github",             // n = 37700
        // "SanFranciscoRoadNetwork",  // n = 174956, m = 223001
        // "NorthAmericaRoadNetwork",  // n = 175813
        // "soc-pokec-relationships",  // n = 1632804, m = 30622565
    ].into_iter().map(|s| (String::from("data/real_graphs/"), String::from(s))).collect();
    bench_trips(c, groupname, &trips, alg);
}

pub fn bench_trips(c: &mut Criterion, groupname: &str, inputs: &Vec<(String,String)>, alg: fn(&UndirectedGraph<u64, BasicEdge<u64>>, usize, usize) -> PathResult<u64, BasicEdge<u64>>) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group(groupname);
    group.sample_size(20);
    for (folder, input) in inputs {
        bench_trip(&mut group, folder, input, alg);
    }
}

pub fn bench_delaunay_graphs(c: &mut Criterion, groupname: &str, alg: fn(&UndirectedGraph<u64, BasicEdge<u64>>, usize, usize) -> PathResult<u64, BasicEdge<u64>>) {
    let mut trips = Vec::new();
    for i in (1000..=100_000).step_by(1000) {
        trips.push((String::from("data/delaunay_graphs/normal_delaunay_graphs/"), ["delaunay", &i.to_string()].concat()));
    }
    bench_trips(c, groupname, &trips, alg);
}

fn parse_bench<W: Weight>(folder: &str, filename: &str) -> (UndirectedGraph<W,BasicEdge<W>>, Vec<(usize,usize)>) {
    println!("{}", [folder, filename, "/", filename, ".in"].concat());
    let graph: UndirectedGraph<W,BasicEdge<W>> = read_to_string([folder, filename, "/", filename, ".in"].concat())
        .expect("Could not find the graph")
        .parse()
        .expect("Could not read the graph");
    let queries = read_to_string([folder, filename, "/", filename, ".path"].concat())
        .expect("Could not find the queries")
        .lines()
        .map(|r| {
            let mut ws = r.split(' ');
            (
                ws.next().unwrap().parse().unwrap(),
                ws.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    (graph, queries)
}
