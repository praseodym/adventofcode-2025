use adventofcode_2025::print_answers;

/// https://adventofcode.com/2025/day/6
fn main() {
    print_answers(run(include_str!("../../input/day06/input")));
}

fn run(input: &'static str) -> (u64, u64) {
    // part 1
    let mut homework = vec![];
    for line in input.lines() {
        let s = line.split_ascii_whitespace();
        let v = s.collect::<Vec<_>>();
        homework.push(v);
    }
    let ops = homework.pop().unwrap();

    let mut part1_answer = 0;
    for i in 0..homework.first().unwrap().len() {
        let mut problem = vec![];
        for line in &homework {
            let number: u64 = line[i].parse().unwrap();
            problem.push(number);
        }
        part1_answer += solve_problem(&problem, ops[i]);
    }

    // part 2
    let mut part2_answer = 0;
    let homework = input
        .lines()
        .map(|l| l.as_bytes())
        .take(homework.len())
        .collect::<Vec<_>>();
    let mut problem: Vec<u64> = vec![];
    let mut problem_count = ops.len() - 1;
    for i in (0..homework.iter().map(|l| l.len()).max().unwrap()).rev() {
        let mut number = String::new();
        for line in &homework {
            if line.len() > i && line[i].is_ascii_digit() {
                number.push(char::from(line[i]));
            }
        }
        if number.is_empty() {
            part2_answer += solve_problem(&problem, ops[problem_count]);
            problem.clear();
            problem_count = problem_count.saturating_sub(1);
            continue;
        }
        problem.push(number.parse().unwrap());
    }
    part2_answer += solve_problem(&problem, ops[problem_count]);

    (part1_answer, part2_answer)
}

fn solve_problem(problem: &[u64], op: &str) -> u64 {
    match op {
        "+" => problem.iter().sum(),
        "*" => problem.iter().product(),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod day06_tests {
    use super::*;

    #[test]
    fn input() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day06/input"));
        assert_eq!(part1_answer, 6605396225322);
        assert_eq!(part2_answer, 11052310600986);
    }

    #[test]
    fn example() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day06/example"));
        assert_eq!(part1_answer, 4277556);
        assert_eq!(part2_answer, 3263827);
    }
}
