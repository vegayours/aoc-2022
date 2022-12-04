use aoc_runner_derive::aoc;
use std::{cmp::Reverse, iter, collections::BinaryHeap};

fn parse_calories(input: &str) -> Vec<i32> {
    let mut calories = Vec::new();
    let lines = input.lines().map(|x| x.trim()).chain(iter::once(""));
    let mut current = 0;
    for line in lines {
        if line.is_empty() {
            calories.push(current);
            current = 0;
        } else {
            current += line.parse::<i32>().unwrap();
        }
    }
    calories
}

#[aoc(day01, part1)]
pub fn part1(input: &str) -> i32 {
    parse_calories(input).into_iter().max().unwrap()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    let mut heap = BinaryHeap::new();
    for x in parse_calories(input) {
        heap.push(Reverse(x));
        if heap.len() > 3 {
            heap.pop();
        }
    }
    heap.into_iter().map(|x| x.0).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let example = include_str!("examples/day01.txt");
        assert_eq!(part1(example), 24000);
    }

    #[test]
    fn test_example_part2() {
        let example = include_str!("examples/day01.txt");
        assert_eq!(part2(example), 45000);
    }
}
