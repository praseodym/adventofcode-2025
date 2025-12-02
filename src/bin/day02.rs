use adventofcode_2025::print_answers;

/// https://adventofcode.com/2025/day/2
fn main() {
    print_answers(run(include_str!("../../input/day02/input")));
}

fn run(input: &'static str) -> (u64, u64) {
    let mut part1_answer = 0;
    let mut part2_answer = 0;

    for r in input.trim().split(",") {
        let mut rs = r.split("-");
        let a: u64 = rs.next().unwrap().parse().unwrap();
        let b: u64 = rs.next().unwrap().parse().unwrap();
        'id: for id in a..=b {
            let id_str = id.to_string();
            let n = id_str.len();
            if id_str[0..n / 2] == id_str[n / 2..] {
                part1_answer += id;
            }
            'i: for i in 1..=n / 2 {
                if n.rem_euclid(i) != 0 {
                    continue 'i;
                }
                for j in 0..(n / i) - 1 {
                    if id_str[j * i..(j + 1) * i] != id_str[(j + 1) * i..(j + 2) * i] {
                        continue 'i;
                    }
                }
                part2_answer += id;
                continue 'id;
            }
        }
    }

    (part1_answer, part2_answer)
}

#[cfg(test)]
mod day02_tests {
    use super::*;

    #[test]
    fn input() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day02/input"));
        assert_eq!(part1_answer, 38158151648);
        assert_eq!(part2_answer, 45283684555);
    }

    #[test]
    fn example() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day02/example"));
        assert_eq!(part1_answer, 1227775554);
        assert_eq!(part2_answer, 4174379265);
    }
}
