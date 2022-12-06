use std::collections::{HashMap, HashSet};

use aoc_runner_derive::aoc;

fn solve(input: &str, window_size: usize) -> usize {
    input
        .as_bytes()
        .windows(window_size)
        .position(|s| s.iter().copied().collect::<HashSet<u8>>().len() == window_size)
        .unwrap()
        + window_size
}

fn solve_imperative(input: &str, window_size: usize) -> usize {
    let mut freq: HashMap<u8, usize> = HashMap::new();
    for c in input.as_bytes()[..window_size - 1].iter() {
        freq.entry(*c).and_modify(|val| *val += 1).or_insert(1);
    }
    let it_prev = input.as_bytes().iter().copied();
    let it_next = input.as_bytes().iter().copied().skip(window_size - 1);
    for (i, (next, prev)) in it_next.zip(it_prev).enumerate() {
        freq.entry(next).and_modify(|val| *val += 1).or_insert(1);
        if freq.len() == window_size {
            return i + window_size;
        }
        if freq.entry(prev).and_modify(|val| *val -= 1).or_default() == &0 {
            freq.remove(&prev);
        }
    }
    panic!("No solution");
}

#[aoc(day06, part1)]
pub fn part1(input: &str) -> usize {
    solve(input, 4)
}

#[aoc(day06, part2)]
pub fn part2(input: &str) -> usize {
    solve(input, 14)
}

#[aoc(day06, part1, imperative)]
pub fn part1_imperative(input: &str) -> usize {
    solve_imperative(input, 4)
}

#[aoc(day06, part2, imperative)]
pub fn part2_imperative(input: &str) -> usize {
    solve_imperative(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let example = include_str!("examples/day06.txt");
        assert_eq!(part1(example), 5);
    }

    #[test]
    fn test_example_part2() {
        let example = include_str!("examples/day06.txt");
        assert_eq!(part2(example), 23);
    }

    #[test]
    fn test_example_part1_imperative() {
        let example = include_str!("examples/day06.txt");
        assert_eq!(part1_imperative(example), 5);
    }

    #[test]
    fn test_example_part2_imperative() {
        let example = include_str!("examples/day06.txt");
        assert_eq!(part2_imperative(example), 23);
    }
}
