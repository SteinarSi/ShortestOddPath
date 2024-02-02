use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};

use shortest_odd_path::algorithm::algorithm::Algorithm;
use shortest_odd_path::algorithm::odd_walk::{BasicOddWalk};
use shortest_odd_path::structure::graph::Graph;
use shortest_odd_path::structure::undirected_graph::UndirectedGraph;
use shortest_odd_path::utility::format::format_input_filepath;

pub fn bench_walks(c: &mut Criterion) {
    let inputs = [("small_graphs", "small1"), ("small_graphs", "small2"), ("small_graphs", "small3")];

    let mut group = c.benchmark_group("bench walks");

    for (folder, file) in inputs {
        let path = format_input_filepath(folder, file);
        let input = std::fs::read_to_string(&path).expect(&format!("Could not find graph: {}", path));
        let graph = UndirectedGraph::from(input);
        let t = graph.n()-1;
        group.bench_function(BenchmarkId::from_parameter(path), |b| {
            b.iter(|| BasicOddWalk::init((graph.clone(), 0, t)).solve());
        });
    }
    group.finish();
}

criterion_group!(benches, bench_walks);
criterion_main!(benches);
