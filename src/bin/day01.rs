use adventofcode_2025::print_answers;

/// https://adventofcode.com/2025/day/1
fn main() {
    print_answers(run(include_str!("../../input/day01/input")));
}

fn run(input: &'static str) -> (u32, u32) {
    let mut part1_answer = 0;
    let mut part2_answer = 0;
    let mut pos: i16 = 50;
    for line in input.lines() {
        let dir = &line[0..1];
        let num = line[1..].parse::<i16>().unwrap();
        for _ in 0..num {
            pos += match dir {
                "L" => -1,
                "R" => 1,
                _ => unreachable!(),
            };
            pos = pos.rem_euclid(100);
            if pos == 0 {
                part2_answer += 1;
            }
        }
        if pos == 0 {
            part1_answer += 1;
        }
    }
    (part1_answer, part2_answer)
}

#[cfg(test)]
mod day01_tests {
    use super::*;

    #[test]
    fn input() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day01/input"));
        assert_eq!(part1_answer, 1100);
        assert_eq!(part2_answer, 6358);
    }

    #[test]
    fn example() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day01/example"));
        assert_eq!(part1_answer, 3);
        assert_eq!(part2_answer, 6);
    }
}
