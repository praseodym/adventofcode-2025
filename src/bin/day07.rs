use adventofcode_2025::{grid::Grid, point::Point, print_answers};

/// https://adventofcode.com/2025/day/7
fn main() {
    print_answers(run(include_str!("../../input/day07/input")));
}

fn run(input: &'static str) -> (u32, u64) {
    let grid = Grid::<u8>::parse_nonsquare(input);
    let mut beams = vec![0; grid.width];
    let mut splits = 0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            match grid[Point::new(x as i32, y as i32)] {
                b'S' => {
                    beams[x] = 1;
                }
                b'^' if beams[x] > 0 => {
                    beams[x - 1] += beams[x];
                    beams[x + 1] += beams[x];
                    beams[x] = 0;
                    splits += 1;
                }
                b'^' | b'.' => {}
                _ => unreachable!(),
            }
        }
    }

    let part1_answer = splits;
    let part2_answer = beams.iter().sum();

    (part1_answer, part2_answer)
}

#[cfg(test)]
mod day07_tests {
    use super::*;

    #[test]
    fn input() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day07/input"));
        assert_eq!(part1_answer, 1698);
        assert_eq!(part2_answer, 95408386769474);
    }

    #[test]
    fn example() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day07/example"));
        assert_eq!(part1_answer, 21);
        assert_eq!(part2_answer, 40);
    }
}
