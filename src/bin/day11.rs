use adventofcode_2025::print_answers;

use itertools::Itertools;
use std::collections::HashMap;

/// https://adventofcode.com/2025/day/11
fn main() {
    print_answers(run(include_str!("../../input/day11/input")));
}

type Node = &'static str;
type Graph = HashMap<Node, Vec<Node>>;

fn run(input: &'static str) -> (u64, u64) {
    let mut graph: Graph = HashMap::new();
    for line in input.lines() {
        let (a, edges) = line.split_once(": ").unwrap();
        let edges = edges.split_ascii_whitespace().collect::<Vec<_>>();
        edges
            .iter()
            .for_each(|b| graph.entry(a).or_default().push(b));
    }

    let mut memo = HashMap::new();
    let part1_answer = num_paths("you", "out", &graph, &mut memo);
    let part2_answer = [["svr", "fft", "dac", "out"], ["svr", "dac", "fft", "out"]]
        .iter()
        .map(|p| {
            p.iter()
                .tuple_windows()
                .map(|(source, sink)| num_paths(source, sink, &graph, &mut memo))
                .product::<u64>()
        })
        .sum();
    (part1_answer, part2_answer)
}

pub fn num_paths(
    source: Node,
    sink: Node,
    graph: &Graph,
    memo: &mut HashMap<(Node, Node), u64>,
) -> u64 {
    if source == sink {
        return 1;
    } else if let Some(&n) = memo.get(&(source, sink)) {
        return n;
    }

    let mut n = 0;
    if let Some(edges) = graph.get(source) {
        for next in edges {
            n += num_paths(next, sink, graph, memo)
        }
    }
    memo.insert((source, sink), n);
    n
}

#[cfg(test)]
mod day11_tests {
    use super::*;

    #[test]
    fn input() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day11/input"));
        assert_eq!(part1_answer, 636);
        assert_eq!(part2_answer, 509312913844956);
    }

    #[test]
    fn example1() {
        let (part1_answer, _) = run(include_str!("../../input/day11/example1"));
        assert_eq!(part1_answer, 5);
    }

    #[test]
    fn example2() {
        let (_, part2_answer) = run(include_str!("../../input/day11/example2"));
        assert_eq!(part2_answer, 2);
    }
}
