use adventofcode_2025::print_answers;
use itertools::Itertools;

/// https://adventofcode.com/2025/day/9
fn main() {
    print_answers(run(include_str!("../../input/day09/input")));
}

fn run(input: &'static str) -> (u64, u64) {
    let mut red_tiles = vec![];
    for line in input.lines() {
        let (x, y) = line.split_once(",").unwrap();
        let x: u64 = x.parse().unwrap();
        let y: u64 = y.parse().unwrap();
        red_tiles.push((x, y));
    }

    let mut part1_answer = 0;
    let mut part2_answer = 0;

    for (a, b) in red_tiles.iter().tuple_combinations() {
        let rectangle = (*a, *b);
        let area = area(rectangle);
        part1_answer = part1_answer.max(area);
        if !has_collision(rectangle, &red_tiles) {
            part2_answer = part2_answer.max(area);
        }
    }

    (part1_answer, part2_answer)
}

type Tile = (u64, u64);
type Rectangle = (Tile, Tile);

pub fn area((a, b): Rectangle) -> u64 {
    (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1)
}

pub fn minmax(v1: u64, v2: u64) -> (u64, u64) {
    // like https://doc.rust-lang.org/std/cmp/fn.minmax.html
    (v1.min(v2), v1.max(v2))
}

pub fn has_collision(((x1, y1), (x2, y2)): Rectangle, tiles: &[Tile]) -> bool {
    let (x1, x2) = minmax(x1, x2);
    let (y1, y2) = minmax(y1, y2);
    for (&(bx1, by1), &(bx2, by2)) in tiles.iter().circular_tuple_windows() {
        let (bx1, bx2) = minmax(bx1, bx2);
        let (by1, by2) = minmax(by1, by2);
        if bx2 > x1 && x2 > bx1 && by2 > y1 && y2 > by1 {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod day09_tests {
    use super::*;

    #[test]
    fn input() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day09/input"));
        assert_eq!(part1_answer, 4748985168);
        assert_eq!(part2_answer, 1550760868);
    }

    #[test]
    fn example() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day09/example"));
        assert_eq!(part1_answer, 50);
        assert_eq!(part2_answer, 24);
    }
}
