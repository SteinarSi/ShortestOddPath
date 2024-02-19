use crate::algorithm::algorithm::{Algorithm, PathResult, PathResult::*, Problem};
use crate::structure::cost::{Cost, Cost::*};
use crate::structure::undirected_graph::UndirectedGraph;
use crate::utility::format::{format_answer_filepath, format_input_filepath};

type Answer = (usize, Cost);

pub fn test_s_t_trip<Alg>(folder: &str, file: &str)
    where Alg: Algorithm,
          <Alg as Algorithm>::Pr: Problem<In = (UndirectedGraph, usize, usize), Out = PathResult>
{
    let path = format_input_filepath(folder, file);
    let input = std::fs::read_to_string(&path).expect(&format!("Could not find graph: {}", path));
    let graph = UndirectedGraph::from(input);
    let answers: Vec<Answer> = parse_answers(std::fs::read_to_string(&format_answer_filepath(folder, file, &Alg::Pr::name()))
        .expect("Answers not found"));

    for (sink, expected) in answers {
        let actual = Alg::init((graph.clone(), 0, sink)).solve();
        match (expected, actual) {
            (Infinite, Possible {cost: _, path}) => panic!("Expected to not find any {}-{}-path, but found one anyway: {:?}", 0, sink, path),
            (Finite(cost), Impossible) => panic!("Expected the alg to find an {}-{}-path of cost {}, but it did not", 0, sink, cost),
            (Finite(cost1), Possible {cost: cost2, path: _}) => assert_eq!(cost1, cost2, "Expected to find an {}-{}-path of cost {}, but found one that costs {} instead", 0, sink, cost1, cost2),
            _ => {}
        }
        println!("Success :-)");
    }
}

fn parse_answers(ans: String) -> Vec<Answer> {
    let mut ret = Vec::new();

    for line in ans.lines() {
        let mut words = line.split(' ');
        let sink = words.next().unwrap().parse().unwrap();
        let cost = words.next().unwrap().parse().unwrap();
        ret.push((sink, cost));
    }

    ret
}
