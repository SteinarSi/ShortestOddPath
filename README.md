## Shortest Odd Path

This is the codebase for my master's thesis in Algorithms at the University of Bergen.

The thesis explains everything and is available both as a [PDF](Diverting%20Networks%20with%20Odd%20Paths.pdf) and in [source code](https://github.com/SteinarSi/DivertingNetworksWithOddPaths).

***

#### Prerequisites
The library is written in Rust, which can be installed [here](https://www.rust-lang.org/learn/get-started).

We use [Python](https://www.python.org/) scripts for generating graphs, benchmarking the algorithms, and plotting the results. In addition, we use `matplotlib`, `networkx`, `numpy` and `scipy`, all of which can be installed by running `pip install matplotlib networkx numpy scipy`.

Although not required, if you are looking for an IDE to explore Rust projects like this one, then I will recommend [RustRover](https://www.jetbrains.com/rust).

#### Build & Run
You may check that the project builds correctly with either `cargo check` or `cargo build`.

Run `cargo test` to run the unit tests.

To run all benchmarks, run `cargo criterion`. To run benchmarks for a specific algorithm, run `cargo criterion --bench <bench>`, where `<bench>` is replaced by either `bench_odd_path`, `bench_odd_walk`, or `bench_network_diversion`. For more fine-grained control of which algorithms are benched, on which graphs, and under what criteria, you may go into the relevant files in the `benches` folder and edit them manually.
