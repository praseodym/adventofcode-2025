use adventofcode_2025::print_answers;

/// https://adventofcode.com/2025/day/8
fn main() {
    print_answers(run(include_str!("../../input/day08/input"), 1000));
}

fn run(input: &'static str, part1_connections: usize) -> (u32, u32) {
    let mut part1_answer = 0;
    let mut part2_answer = 0;

    let mut boxes = vec![];
    for (i, line) in input.lines().enumerate() {
        let mut s = line.split(",");
        let point = JunctionBox {
            parent: i,
            x: s.next().unwrap().parse().unwrap(),
            y: s.next().unwrap().parse().unwrap(),
            z: s.next().unwrap().parse().unwrap(),
        };
        boxes.push(point);
    }

    let mut dist = Vec::<(u64, usize, usize)>::new();
    for (i, a) in boxes.iter().enumerate() {
        for (j, b) in boxes.iter().enumerate().skip(i) {
            if i == j {
                continue;
            }
            dist.push((a.distance(b), i, j));
        }
    }
    dist.sort_by(|a, b| a.0.cmp(&b.0));
    dist.reverse();

    let mut circuit_sizes = vec![1_usize; boxes.len()];
    let mut num_connections = 0;
    while let Some(closest) = dist.pop() {
        num_connections += 1;
        let a = boxes[closest.1].clone();
        let b = boxes[closest.2].clone();
        if a.parent != b.parent {
            boxes
                .iter_mut()
                .filter(|x| x.parent == b.parent)
                .for_each(|x| x.parent = a.parent);
            circuit_sizes[a.parent] += circuit_sizes[b.parent];
            circuit_sizes[b.parent] = 0;
        }
        if num_connections == part1_connections {
            let mut circuit_sizes = circuit_sizes.clone();
            circuit_sizes.sort();
            part1_answer = circuit_sizes.iter().rev().take(3).product::<usize>() as u32;
        }
        if circuit_sizes[a.parent] == boxes.len() {
            part2_answer = a.x * b.x;
            break;
        }
    }

    (part1_answer, part2_answer)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct JunctionBox {
    parent: usize,
    x: u32,
    y: u32,
    z: u32,
}

impl JunctionBox {
    pub fn distance(&self, other: &JunctionBox) -> u64 {
        ((self.x as i64 - other.x as i64).pow(2)
            + (self.y as i64 - other.y as i64).pow(2)
            + (self.z as i64 - other.z as i64).pow(2)) as u64
    }
}

#[cfg(test)]
mod day08_tests {
    use super::*;

    #[test]
    fn input() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day08/input"), 1000);
        assert_eq!(part1_answer, 54180);
        assert_eq!(part2_answer, 25325968);
    }

    #[test]
    fn example() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day08/example"), 10);
        assert_eq!(part1_answer, 40);
        assert_eq!(part2_answer, 25272);
    }
}
