use std::fs::read_to_string;
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkGroup, BenchmarkId};
use criterion::measurement::WallTime;
use shortest_odd_path::algorithm::network_diversion::network_diversion;
use shortest_odd_path::structure::graph::planar_graph::PlanarGraph;

fn bench_network_diversion(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("Bench Network Diversion");
    group.sample_size(10);
    let inputs = [
        "CityOfOldenburg", // n = 6105
        "CityOfSanJoaquinCounty", // n = 18263
        "CaliforniaRoadNetwork", // n = 21048
        // "SanFranciscoRoadNetwork", // n = 174956
    ];
    for input in inputs {
        let (graph, queries) = parse_bench(input);
        for (i, (s,t,(du,dv))) in queries.iter().enumerate() {
            group.bench_function(BenchmarkId::from_parameter([input, "query #", i.to_string().as_str()].concat()), |b| {
                println!();
                b.iter(|| network_diversion(&graph, *s, *t, (*du,*dv)));
            });
        }
    }
}

fn parse_bench(filename: &str) -> (PlanarGraph<f64>, Vec<(usize, usize, (usize,usize))>) {
    let graph: PlanarGraph<f64> = read_to_string(["data/planar_graphs/real_planar_graphs/", filename, "/", filename, ".in"].concat())
        .expect("Could not find the graph")
        .parse()
        .expect("Could not read the graph");
    let queries = read_to_string(["data/planar_graphs/real_planar_graphs/", filename, "/", filename, ".diversion"].concat())
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

criterion_group!(benches, bench_network_diversion);
criterion_main!(benches);
