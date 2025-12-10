use std::collections::VecDeque;

use adventofcode_2025::print_answers;
use good_lp::{
    Expression, IntoAffineExpression, Solution, SolverModel, solvers::coin_cbc::coin_cbc, variable,
    variables,
};

/// https://adventofcode.com/2025/day/10
fn main() {
    print_answers(run(include_str!("../../input/day10/input")));
}

fn run(input: &'static str) -> (u32, u32) {
    let mut part1_answer = 0;
    let mut part2_answer = 0;

    for line in input.lines() {
        let s = line.split_once("]").unwrap();
        let goal_str = &s.0[1..];
        let s = s.1.split_once("{").unwrap();
        let buttons_str = s.0.trim();
        let joltages_str = &s.1[..s.1.len() - 1];

        let goal: String = goal_str
            .as_bytes()
            .iter()
            .map(|&b| if b == b'#' { "1" } else { "0" })
            .rev()
            .collect();
        let goal = usize::from_str_radix(&goal, 2).unwrap();

        let mut buttons_int = vec![];
        let mut buttons_binary = vec![];
        for button_str in buttons_str.split_ascii_whitespace() {
            let button_int = button_str[1..button_str.len() - 1]
                .split(",")
                .map(|w| w.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            let mut button_binary: usize = 0;
            button_int.iter().for_each(|w| button_binary |= 1 << w);

            buttons_int.push(button_int);
            buttons_binary.push(button_binary);
        }

        part1_answer += solve_button_lights(&buttons_binary, goal);

        let joltages = joltages_str
            .split(",")
            .map(|j| j.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        part2_answer += solve_button_joltages(&buttons_int, &joltages);
    }

    (part1_answer, part2_answer)
}

fn solve_button_lights(buttons: &[usize], goal: usize) -> u32 {
    let mut q = VecDeque::new();
    q.push_back((0_usize, 0));
    while let Some((a, i)) = q.pop_front() {
        for b in buttons {
            let i = i + 1;
            let c = a ^ b;
            if c == goal {
                return i;
            } else {
                q.push_back((c, i));
            }
        }
    }
    u32::MAX
}

fn solve_button_joltages(buttons: &[Vec<usize>], joltages: &[u32]) -> u32 {
    let mut vars = variables!();
    let button_press_vars = buttons
        .iter()
        .map(|_| vars.add(variable().min(0).integer()))
        .collect::<Vec<_>>();
    let objective = button_press_vars.iter().sum::<Expression>();
    let mut pb = vars.minimise(objective).using(coin_cbc);
    let mut joltage_expressions = vec![0_u32.into_expression(); joltages.len()];
    for (i, button) in buttons.iter().enumerate() {
        for &j in button {
            joltage_expressions[j] += button_press_vars[i];
        }
    }
    for (i, expr) in joltage_expressions.into_iter().enumerate() {
        pb.add_constraint(expr.eq(joltages[i]));
    }
    let sol = pb.solve().unwrap();
    let min_button_presses = button_press_vars
        .iter()
        .map(|&var| sol.value(var))
        .sum::<f64>();
    min_button_presses as u32
}

#[cfg(test)]
mod day10_tests {
    use super::*;

    #[test]
    fn input() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day10/input"));
        assert_eq!(part1_answer, 484);
        assert_eq!(part2_answer, 19210);
    }

    #[test]
    fn example() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day10/example"));
        assert_eq!(part1_answer, 7);
        assert_eq!(part2_answer, 33);
    }
}
