use shortest_odd_path::algorithm::algorithm::PathResult::*;
use shortest_odd_path::algorithm::bottleneck_path::shortest_bottleneck_path;
use shortest_odd_path::structure::cost::{Cost, Cost::*};
use shortest_odd_path::structure::undirected_graph::UndirectedGraph;

fn test_bottleneck_path(path: &str) {
    let p = format_input_filepath(path);
    println!("{}", p);
    let graph = UndirectedGraph::from(std::fs::read_to_string(format_input_filepath(path)).unwrap());
    let answers = parse_answers(std::fs::read_to_string(format_answer_filepath(path)).unwrap());

    for (source, sink, (u,v), expected) in answers {
        let actual = shortest_bottleneck_path(&graph, source, sink, (u,v));
        match (expected, actual) {
            (Infinite, Possible{cost: _, path: _}) => panic!("Expected not to find a path, but the alg did anyway!"),
            (Finite(c), Impossible) => panic!("Expected a path of cost {}, but the alg couldn't find it!", c),
            (Finite(c), Possible{cost, path}) => {
                assert_eq!(c, cost);
                assert!((0..path.len()-1).find(|&i| (path[i], path[i+1]) == (u,v)).is_some());
            },
            _ => {},
        }
    }
}

fn parse_answers(answers: String) -> Vec<(usize, usize, (usize,usize), Cost)> {
    answers.lines().map(|l| {
        let mut ws = l.split(' ');
        let s = ws.next().unwrap().parse().unwrap();
        let t = ws.next().unwrap().parse().unwrap();
        let u = ws.next().unwrap().parse().unwrap();
        let v = ws.next().unwrap().parse().unwrap();
        let d = Cost::from(ws.next().unwrap().parse());
        (s, t, (u,v), d)
    }).collect()
}

fn format_input_filepath(path: &str) -> String {
    [path, ".in"].concat()
}

fn format_answer_filepath(path: &str) -> String {
    [path, ".bottleneck"].concat()
}

#[test]
fn medium1() {
    test_bottleneck_path("data/medium_graphs/medium1/medium1");
}