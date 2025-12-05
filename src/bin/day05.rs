use std::ops::RangeInclusive;

use adventofcode_2025::print_answers;

/// https://adventofcode.com/2025/day/5
fn main() {
    print_answers(run(include_str!("../../input/day05/input")));
}

fn run(input: &'static str) -> (u64, u64) {
    let s = input.split_once("\n\n").unwrap();
    let ranges = s.0;
    let ingredients = s.1;

    let mut ranges = ranges
        .lines()
        .map(|range| {
            let (a, b) = range.split_once("-").unwrap();
            let a = a.parse::<u64>().unwrap();
            let b = b.parse::<u64>().unwrap();
            a..=b
        })
        .collect::<Vec<_>>();

    let ingredients = ingredients
        .lines()
        .map(|i| i.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut part1_answer = 0;

    'ingredients: for ingredient in ingredients {
        for range in &ranges {
            if range.contains(&ingredient) {
                part1_answer += 1;
                continue 'ingredients;
            }
        }
    }

    let mut part2_answer = 0;

    ranges.sort_by(|a, b| a.start().cmp(b.start()));
    let mut ranges_dedup = Vec::<RangeInclusive<u64>>::new();
    for range in &ranges {
        if let Some(prev) = ranges_dedup.last_mut()
            && prev.contains(range.start())
        {
            if !prev.contains(range.end()) {
                *prev = *prev.start()..=*range.end()
            }
        } else {
            ranges_dedup.push(range.clone());
        }
    }

    for range in ranges_dedup {
        part2_answer += range.end() + 1 - range.start();
    }

    (part1_answer, part2_answer)
}

#[cfg(test)]
mod day05_tests {
    use super::*;

    #[test]
    fn input() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day05/input"));
        assert_eq!(part1_answer, 707);
        assert_eq!(part2_answer, 361615643045059);
    }

    #[test]
    fn example() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day05/example"));
        assert_eq!(part1_answer, 3);
        assert_eq!(part2_answer, 14);
    }
}
