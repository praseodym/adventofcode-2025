use adventofcode_2025::print_answers;

/// https://adventofcode.com/2025/day/3
fn main() {
    print_answers(run(include_str!("../../input/day03/input")));
}

fn run(input: &'static str) -> (u64, u64) {
    let mut part1_answer = 0;
    let mut part2_answer = 0;
    for line in input.lines() {
        let bank: Vec<u8> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        part1_answer += bank_max(&bank, 2);
        part2_answer += bank_max(&bank, 12);
    }
    (part1_answer, part2_answer)
}

fn bank_max(bank: &[u8], num: usize) -> u64 {
    let mut bank_max: u64 = 0;
    let mut n = 0;
    for i in (0..num).rev() {
        // last i batteries in the bank are reserved for later
        let m = bank.len() - i;
        let bank_slice = &bank[n..m];
        let max = *bank_slice.iter().max().unwrap();
        // first n batteries can no longer be used
        n += bank_slice.iter().position(|b| *b == max).unwrap() + 1;
        bank_max += 10_u64.pow(i as u32) * max as u64;
    }
    bank_max
}

#[cfg(test)]
mod day03_tests {
    use super::*;

    #[test]
    fn input() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day03/input"));
        assert_eq!(part1_answer, 17278);
        assert_eq!(part2_answer, 171528556468625);
    }

    #[test]
    fn example() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day03/example"));
        assert_eq!(part1_answer, 357);
        assert_eq!(part2_answer, 3121910778619);
    }
}
