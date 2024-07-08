use std::collections::BTreeSet;
use crate::structure::graph::edge::Edge;
use crate::structure::weight::Weight;

pub fn repeat<E>(n: usize, e: E) -> Vec<E>
    where E: Clone
{
    (0..n).map(|_| e.clone()).collect()
}

const DEBUG_MODE: bool = false;
pub fn debug(s: String) {
    if DEBUG_MODE {
        println!("{}", s);
    }
}

pub fn assert_is_path<W: Weight, E: Edge<W>>(path: &Vec<E>) {
    if let Some(f) = path.first() {
        let mut visited = BTreeSet::new();
        visited.insert(f.from());
        path.iter().for_each(|e| {
            assert!( ! visited.contains(&e.to()), "The 'path' visits {} multiple times, and is not a path!", e.to());
            visited.insert(e.to());
        });
    }
}
