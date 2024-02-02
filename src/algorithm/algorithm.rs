use crate::structure::undirected_graph::UndirectedGraph;

pub trait Problem{
    type In;
    type Out;
    fn name() -> String;
}

pub trait Algorithm {
    type Pr: Problem;
    fn init(input: <Self::Pr as Problem>::In) -> Self where Self: Sized;
    fn solve(&mut self) -> <Self::Pr as Problem>::Out;
}

pub enum PathResult {
    Possible {
        cost: u64,
        path: Vec<usize>,
    },
    Impossible
}
pub struct ShortestOddWalk;
impl Problem for ShortestOddWalk {
    type In = (UndirectedGraph, usize, usize);
    type Out = PathResult;
    fn name() -> String {
        String::from("walk")
    }
}

pub struct ShortestOddPath;
impl Problem for ShortestOddPath {
    type In = (UndirectedGraph, usize, usize);
    type Out = PathResult;
    fn name() -> String { String::from("path") }
}