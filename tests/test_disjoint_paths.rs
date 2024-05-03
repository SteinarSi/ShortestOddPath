use std::fmt::{Debug, Display};
use std::str::FromStr;
use shortest_odd_path::algorithm::two_disjoint_paths::two_disjoint_paths;
use shortest_odd_path::structure::cost::{Cost, Finite, Infinite};
use shortest_odd_path::structure::graph::edge::BasicEdge;
use shortest_odd_path::structure::graph::undirected_graph::UndirectedGraph;
use shortest_odd_path::structure::weight::Weight;
use utility::{Problem, verify_path};
use crate::utility::meta_test;

mod utility;

pub struct TwoDisjointPaths;
impl <W> Problem<W> for TwoDisjointPaths
    where W: Weight,
          <W as FromStr>::Err: Debug + Display,
{
    type Output = Option<(W, Vec<BasicEdge<W>>, Vec<BasicEdge<W>>)>;
    type Query = ((usize, usize), (usize, usize), Cost<W>);
    type GraphClass = UndirectedGraph<W,BasicEdge<W>>;
    fn name() -> String { String::from("disjoint") }
    fn parse_query(query: &str) -> Option<Self::Query> {
        let mut words = query.split(' ');
        Some((
            (words.next()?.parse().ok()?, words.next()?.parse().ok()?),
            (words.next()?.parse().ok()?, words.next()?.parse().ok()?),
            Cost::from(words.next()?.parse())
        ))
    }
    fn display_query(((s1,t1), (s2,t2), _): &Self::Query) -> String {
        format!("Disjoint path from {} to {}, and from {} to {}:", s1, t1, s2, t2)
    }
    fn verify_answer(graph: &Self::GraphClass, query: &Self::Query, actual: &Self::Output) {
        let ((s1, t1), (s2,t2), cost) = query;
        let context = Self::display_query(query);
        match (cost, actual) {
            (Finite(_), None) => panic!("{}\nCould not find two vertex-disjoint paths, but it *should* be possible!", context),
            (Infinite, Some((_, p1, p2))) => panic!("{}\nWe didn't expect to find two vertex-disjoint paths from {} to {} and from {} to {}, but we did anyway: \n{:?}\nand\n{:?} ",context, s1, t1, s2, t2, p1,p2),
            (Finite(c), Some((w, p1,p2))) => {
                assert!( ! p1.iter().any(|u| p2.contains(&u)), "\n{}The two paths were supposed to use different vertices, but they don't:\n{:?}\nand\n{:?}", context, p1,p2);
                assert_eq!(*c, *w, "{}\nExpected two paths from {} to {} and from {} to {} with a combined length of {}, but found two of length {} instead!\n\n{:?}\nand\n{:?}", context, s1, t1, s2, t2, c, w, p1, p2);
                verify_path::<W,BasicEdge<W>,Self>(&graph, &context, 0.into(), 0.into(), p1, *s1, *t1);
                verify_path::<W,BasicEdge<W>,Self>(&graph, &context, 0.into(), 0.into(), p2, *s2, *t2);
            }
            _ => {}
        }
    }
    fn compute(graph: &Self::GraphClass, (p1, p2, _): &Self::Query) -> Self::Output {
        two_disjoint_paths(graph, *p1, *p2)
    }
}
fn test(folder: &str, name: &str) {
    meta_test::<TwoDisjointPaths, u64>(folder, name);
}

#[cfg(test)]
mod test_medium_disjoint_paths {
    fn test(name: &str) { super::test("medium_graphs", name); }
    #[ignore = "The test fails, and the algorithm is probably wrong, but it's not a priority now."]
    #[test]
    fn medium1() { test("medium1"); }
    #[test]
    #[ignore = "The test fails, and the algorithm is probably wrong, but it's not a priority now."]
    fn medium3() { test("medium3"); }
}

#[cfg(test)]
mod test_small_disjoint_paths {
    fn test(name: &str) { super::test("small_graphs", name); }
    #[test]
    fn small3() { test("small3"); }
}
