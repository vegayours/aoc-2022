use std::collections::HashSet;

use aoc_runner_derive::aoc;

fn common_letter(input: &str) -> char {
    let (left, right) = input.split_at(input.len() / 2);
    let left_chars: HashSet<char> = left.chars().collect();
    for c in right.chars() {
        if left_chars.contains(&c) {
            return c;
        }
    }
    panic!("No common letter");
}

fn common_badge(groups: &[&str]) -> char {
    let mut common: HashSet<char> = groups[0].chars().collect();
    for group in &groups[1..] {
        let mut next = HashSet::new();
        for c in group.chars() {
            if common.contains(&c) {
                next.insert(c);
            }
        }
        common = next;
    }
    assert!(common.len() == 1);
    common.into_iter().next().unwrap()
}

fn letter_priority(c: char) -> u32 {
    match c as u32 {
        x if x >= ('a' as u32) && x <= ('z' as u32) => x - ('a' as u32) + 1,
        x if x >= ('A' as u32) && x <= ('Z' as u32) => x - ('A' as u32) + 27,
        _ => panic!("Invalid char: {c}"),
    }
}

#[aoc(day03, part1)]
pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| letter_priority(common_letter(line.trim())))
        .sum::<u32>()
}

#[aoc(day03, part2)]
pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(str::trim)
        .collect::<Vec<_>>()
        .chunks_exact(3)
        .map(common_badge)
        .map(letter_priority)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let example = include_str!("examples/day03.txt");
        assert_eq!(part1(example), 157);
    }

    #[test]
    fn test_example_part2() {
        let example = include_str!("examples/day03.txt");
        assert_eq!(part2(example), 70);
    }
}
