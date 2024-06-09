use criterion::{Criterion, criterion_group, criterion_main};
use shortest_odd_path::algorithm::odd_walk::shortest_odd_walk;
use crate::utility::bench_real_trips;

mod utility;

fn bench_walks(c: &mut Criterion) {
    bench_real_trips(c, "odd walks", shortest_odd_walk);
}

criterion_group!(benches, bench_walks);
criterion_main!(benches);