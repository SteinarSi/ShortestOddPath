mod utility;

use criterion::{criterion_group, criterion_main, Criterion};
use crate::utility::{bench_delaunay_graphs, bench_real_trips};
use shortest_odd_path::algorithm::shortest_path::shortest_path;

fn bench_paths(c: &mut Criterion) {
    // bench_real_trips(c, "BenchRealDijkstra", shortest_path);
    bench_delaunay_graphs(c, "BenchDelaunayDijkstra", shortest_path);
}


criterion_group!(benches, bench_paths);
criterion_main!(benches);
