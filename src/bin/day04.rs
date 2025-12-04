use adventofcode_2025::{grid::Grid, point::ALL_ADJACENT, print_answers};

/// https://adventofcode.com/2025/day/4
fn main() {
    print_answers(run(include_str!("../../input/day04/input")));
}

fn run(input: &'static str) -> (u32, u32) {
    let mut part1_answer = 0;
    let mut part2_answer = 0;

    let mut grid = Grid::parse_bool_square(input, b'@');

    loop {
        let mut accessible: u32 = 0;
        let mut next_grid = grid.clone();
        for (idx, &has_roll) in grid.iter().enumerate() {
            if has_roll {
                let p = grid.point_from_index(idx);
                let adjacent_rolls: usize = ALL_ADJACENT
                    .iter()
                    .filter(|&&d| grid.contains(p + d) && grid[p + d])
                    .count();
                if adjacent_rolls < 4 {
                    accessible += 1;
                    next_grid[p] = false;
                }
            }
        }
        if accessible == 0 {
            break;
        }
        if part1_answer == 0 {
            part1_answer += accessible;
        }
        part2_answer += accessible;
        grid = next_grid;
    }

    (part1_answer, part2_answer)
}

#[cfg(test)]
mod day04_tests {
    use super::*;

    #[test]
    fn input() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day04/input"));
        assert_eq!(part1_answer, 1626);
        assert_eq!(part2_answer, 9173);
    }

    #[test]
    fn example() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day04/example"));
        assert_eq!(part1_answer, 13);
        assert_eq!(part2_answer, 43);
    }
}
