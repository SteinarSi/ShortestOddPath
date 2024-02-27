pub enum PathResult {
    Possible {
        cost: u64,
        path: Vec<usize>,
    },
    Impossible
}