use std::fs::read_to_string;
use std::time::Duration;
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkGroup, BenchmarkId};
use criterion::measurement::WallTime;
use shortest_odd_path::algorithm::network_diversion::network_diversion;
use shortest_odd_path::structure::graph::planar_graph::PlanarGraph;

fn bench_network_diversion(c: &mut Criterion, inputs: Vec<(String,String)>) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("Bench Network Diversion");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    for (folder, file) in inputs {
        let (graph, queries) = parse_bench(&folder, &file);
        for (i, (s,t,(du,dv))) in queries.iter().enumerate() {
            group.bench_function(BenchmarkId::from_parameter(["network diversion - ", &file, ", query #", i.to_string().as_str()].concat()), |b| {
                b.iter(|| network_diversion(&graph, *s, *t, (*du,*dv)));
            });
        }
    }
}

fn divert_real_networks(c: &mut Criterion) {
    let inputs = vec![
        "CityOfOldenburg", // n = 6105, m = 7035
    ].into_iter()
        .map(|filename| ("data/planar_graphs/real_planar_graphs/".to_string(), filename.to_string()))
        .collect();
    bench_network_diversion(c, inputs);
}

fn divert_delaunay_graphs(c: &mut Criterion) {
    let inputs = (1000..=200_000)
        .step_by(1000)
        .map(|i| ("data/delaunay_graphs/planar_delaunay_graphs/".to_string(), ["delaunay", &i.to_string()].concat()))
        .collect();
    bench_network_diversion(c, inputs);
}

fn parse_bench(folder: &str, filename: &str) -> (PlanarGraph<f64>, Vec<(usize, usize, (usize,usize))>) {
    let graph: PlanarGraph<f64> = read_to_string([folder, filename, "/", filename, ".in"].concat())
        .expect("Could not find the graph")
        .parse()
        .expect("Could not read the graph");
    let queries = read_to_string([folder, filename, "/", filename, ".diversion"].concat())
        .expect("Could not find the queries")
        .lines()
        .map(|r| {
            let mut ws = r.split(' ');
            (
                ws.next().unwrap().parse().unwrap(),
                ws.next().unwrap().parse().unwrap(),
                (
                    ws.next().unwrap().parse().unwrap(),
                    ws.next().unwrap().parse().unwrap(),
                ),
            )
        })
        .collect();

    (graph, queries)
}

// criterion_group!(benches, divert_real_networks);
criterion_group!(benches, divert_delaunay_graphs);
criterion_main!(benches);
