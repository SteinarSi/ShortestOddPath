mod utility;

use criterion::{criterion_group, criterion_main, Criterion};
use shortest_odd_path::algorithm::odd_path::shortest_odd_path;
use shortest_odd_path::structure::graph::Graph;
use crate::utility::{bench_trips};

fn bench_paths(c: &mut Criterion) {
    bench_trips(c, "bench odd paths", shortest_odd_path);
}

criterion_group!(benches, bench_paths);
criterion_main!(benches);
