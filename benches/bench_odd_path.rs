mod utility;

use criterion::{criterion_group, criterion_main, Criterion};
use shortest_odd_path::algorithm::odd_path::shortest_odd_path;
use crate::utility::{bench_delaunay_graphs, bench_real_trips};

fn bench_paths(c: &mut Criterion) {
    bench_real_trips(c, "BenchOddRealPaths", shortest_odd_path);
    bench_delaunay_graphs(c, "BenchOddDelaunayPaths", shortest_odd_path);
}


criterion_group!(benches, bench_paths);
criterion_main!(benches);
