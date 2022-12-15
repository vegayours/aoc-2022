use aoc_runner_derive::aoc;

extern crate gcollections;
extern crate interval;

use gcollections::ops::*;
use interval::interval_set::*;
use interval::ops::*;
use regex::Regex;

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();
}

type Coord = (isize, isize);

#[derive(Debug)]
struct Sensor {
    pos: Coord,
    beacon_pos: Coord,
}

impl Sensor {
    fn parse(input: &str) -> Sensor {
        let captures = RE.captures(input).unwrap();
        let parse_capture = |index| {
            captures
                .get(index)
                .unwrap()
                .as_str()
                .parse::<isize>()
                .unwrap()
        };
        Sensor {
            pos: (parse_capture(1), parse_capture(2)),
            beacon_pos: (parse_capture(3), parse_capture(4)),
        }
    }

    fn beacon_distance(&self) -> isize {
        (self.pos.0 - self.beacon_pos.0).abs() + (self.pos.1 - self.beacon_pos.1).abs()
    }
}

fn blocked_interval(sensor: &Sensor, y_pos: isize) -> Option<IntervalSet<isize>> {
    let max_distance = sensor.beacon_distance();
    let (s_x, s_y) = sensor.pos;
    let y_distance = (s_y - y_pos).abs();
    if y_distance > max_distance {
        None
    } else {
        let x_distance = max_distance - y_distance;
        Some(IntervalSet::new(s_x - x_distance, s_x + x_distance))
    }
}

fn blocked_intervals(sensors: &[Sensor], y_pos: isize) -> IntervalSet<isize> {
    sensors
        .iter()
        .flat_map(|s| blocked_interval(s, y_pos))
        .fold(IntervalSet::empty(), |acc, next| acc.union(&next))
}

fn part1_impl(input: &str, y_pos: isize) -> usize {
    let sensors: Vec<Sensor> = input.lines().map(Sensor::parse).collect();
    let blocked_intervals = blocked_intervals(&sensors, y_pos);
    let beacon_intervals = sensors
        .iter()
        .flat_map(|s| {
            let (s_x, s_y) = s.beacon_pos;
            if s_y == y_pos {
                Some(IntervalSet::singleton(s_x))
            } else {
                None
            }
        })
        .fold(IntervalSet::empty(), |acc, sensor| acc.union(&sensor));

    blocked_intervals
        .difference(&beacon_intervals)
        .iter()
        .map(|interval| interval.size())
        .sum()
}

#[aoc(day15, part1)]
pub fn part1(input: &str) -> usize {
    part1_impl(input, 2000000)
}

fn part2_impl(input: &str, limit: isize) -> isize {
    let sensors: Vec<Sensor> = input.lines().map(Sensor::parse).collect();
    for y in 0..=limit {
        let blocked = blocked_intervals(&sensors, y);
        let allowed = IntervalSet::new(0, limit).difference(&blocked);
        if allowed.interval_count() == 1 {
            let allowed_interval = allowed.iter().next().unwrap();
            if allowed_interval.size() == 1 {
                let x = allowed_interval.lower();
                return x * 4_000_000 + y;
            }
        }
    }
    panic!("No solution")
}

#[aoc(day15, part2)]
pub fn part2(input: &str) -> isize {
    part2_impl(input, 4_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let example = include_str!("examples/day15.txt");
        assert_eq!(part1_impl(example, 10), 26);
    }

    #[test]
    fn test_example_part2() {
        let example = include_str!("examples/day15.txt");
        assert_eq!(part2_impl(example, 20), 56000011);
    }
}
