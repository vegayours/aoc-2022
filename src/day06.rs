use std::collections::HashSet;

use aoc_runner_derive::aoc;

fn solve(input: &str, window_size: usize) -> usize {
    input
        .as_bytes()
        .windows(window_size)
        .position(|s| s.iter().copied().collect::<HashSet<u8>>().len() == window_size)
        .unwrap()
        + window_size
}

#[aoc(day06, part1)]
pub fn part1(input: &str) -> usize {
    solve(input, 4)
}

#[aoc(day06, part2)]
pub fn part2(input: &str) -> usize {
    solve(input, 14)
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
}
