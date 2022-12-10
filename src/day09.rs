use std::collections::HashSet;

use aoc_runner_derive::aoc;

#[derive(Debug)]
struct Move {
    dir: u8,
    step: i32,
}

fn parse_moves(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(str::trim)
        .map(
            |l| match l.split_whitespace().collect::<Vec<&str>>().as_slice() {
                [dir, step] => Move {
                    dir: dir.as_bytes()[0],
                    step: step.parse().unwrap(),
                },
                _ => panic!(""),
            },
        )
        .collect()
}

#[derive(Hash, Default, Clone, Debug, Eq, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn distance(&self, other: &Self) -> i32 {
        std::cmp::max((self.x - other.x).abs(), (self.y - other.y).abs())
    }

    fn advance_step(&mut self, to: &Self) {
        self.x += (to.x - self.x).signum();
        self.y += (to.y - self.y).signum();
    }

    fn apply_step(&mut self, m: &Move) {
        match m.dir {
            b'U' => self.x += 1,
            b'D' => self.x -= 1,
            b'R' => self.y += 1,
            b'L' => self.y -= 1,
            d => panic!("Unknown dir: {}", d),
        };
    }
}

fn solve(input: &str, chain_len: usize) -> usize {
    let mut visited: HashSet<Pos> = HashSet::new();
    let mut chain = vec![Pos::default(); chain_len];
    visited.insert(Pos::default());
    for m in parse_moves(input) {
        for _ in 0..m.step {
            chain[0].apply_step(&m);
            for i in 1..chain_len {
                let (prev, next) = chain.split_at_mut(i);
                let (prev, next) = (&mut prev[i - 1], &mut next[0]);
                if next.distance(prev) > 1 {
                    next.advance_step(prev);
                } else {
                    break;
                }
            }
            visited.insert(chain.last().unwrap().clone());
        }
    }
    visited.len()
}

#[aoc(day09, part1)]
pub fn part1(input: &str) -> usize {
    solve(input, 2)
}

#[aoc(day09, part2)]
pub fn part2(input: &str) -> usize {
    solve(input, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let example = include_str!("examples/day09.txt");
        assert_eq!(part1(example), 13);
    }

    #[test]
    fn test_example_part2() {
        let example = include_str!("examples/day09.txt");
        assert_eq!(part2(example), 1);
    }

    #[test]
    fn test_example_1_part2() {
        let example = r#"R 5
    U 8
    L 8
    D 3
    R 17
    D 10
    L 25
    U 20"#;
        assert_eq!(part2(example), 36);
    }
}
