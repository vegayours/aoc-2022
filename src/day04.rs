use std::ops::RangeInclusive;

use aoc_runner_derive::aoc;

struct Assignment {
    first: RangeInclusive<u32>,
    second: RangeInclusive<u32>,
}

fn parse_range(range: &str) -> RangeInclusive<u32> {
    match range.split("-").collect::<Vec<_>>().as_slice() {
        &[start, end] => RangeInclusive::new(start.parse().unwrap(), end.parse().unwrap()),
        _ => panic!("Invalid range: {range}"),
    }
}

impl Assignment {
    fn parse(input: &str) -> Assignment {
        match input.split(",").collect::<Vec<_>>().as_slice() {
            &[first, second] => Assignment {
                first: parse_range(first),
                second: parse_range(second),
            },
            _ => panic!("Invalid assignment: {input}"),
        }
    }
    fn is_fully_contained(&self) -> bool {
        self.first.contains(self.second.start()) && self.first.contains(self.second.end())
            || self.second.contains(self.first.start()) && self.second.contains(self.first.end())
    }

    fn overlaps(&self) -> bool {
        self.first.contains(self.second.start())
            || self.first.contains(self.second.end())
            || self.second.contains(self.first.start())
            || self.second.contains(self.first.end())
    }
}

#[aoc(day04, part1, imperative)]
pub fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(str::trim)
        .map(Assignment::parse)
        .filter(Assignment::is_fully_contained)
        .count() as i32
}

#[aoc(day04, part2)]
pub fn part2(input: &str) -> i32 {
    input
        .lines()
        .map(str::trim)
        .map(Assignment::parse)
        .filter(Assignment::overlaps)
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let example = include_str!("examples/day04.txt");
        assert_eq!(part1(example), 2);
    }

    #[test]
    fn test_example_part2() {
        let example = include_str!("examples/day04.txt");
        assert_eq!(part2(example), 4);
    }
}
