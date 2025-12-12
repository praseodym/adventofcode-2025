/// https://adventofcode.com/2025/day/12
fn main() {
    println!(
        "part 1 answer: {}",
        run(include_str!("../../input/day12/input"))
    );
}

fn run(input: &'static str) -> u32 {
    let mut part1_answer = 0;

    let s = input.split("\n\n").collect::<Vec<_>>();
    let shapes = s
        .iter()
        .take(&s.len() - 1)
        .map(|p| p.bytes().filter(|&b| b == b'#').count() as u32)
        .collect::<Vec<u32>>();

    for region in s.iter().last().unwrap().lines() {
        let (size, quantities) = region.split_once(": ").unwrap();
        let (width, height) = size.split_once("x").unwrap();
        let area = width.parse::<u32>().unwrap() * height.parse::<u32>().unwrap();

        let acc: u32 = quantities
            .split_ascii_whitespace()
            .map(|q| q.parse::<u32>().unwrap())
            .zip(&shapes)
            .map(|(q, s)| q * s)
            .sum();

        if acc <= area {
            part1_answer += 1;
        }
    }

    part1_answer
}

#[cfg(test)]
mod day12_tests {
    use super::*;

    #[test]
    fn input() {
        let part1_answer = run(include_str!("../../input/day12/input"));
        assert_eq!(part1_answer, 505);
    }
}
