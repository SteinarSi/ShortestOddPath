use crate::structure::weight::Weight;

pub enum PathResult<W: Weight> {
    Possible {
        cost: W,
        path: Vec<usize>,
    },
    Impossible
}