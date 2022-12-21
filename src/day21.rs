use std::collections::HashMap;

use aoc_runner_derive::aoc;
use itertools::Itertools;

#[derive(Debug)]
enum Input<'a> {
    Number {
        val: i64,
    },
    Op {
        left: &'a str,
        right: &'a str,
        op: &'a str,
    },
}

impl<'a> Input<'a> {
    fn parse(input: &'a str) -> HashMap<&'a str, Input<'a>> {
        input
            .lines()
            .map(|l| {
                let parts: Vec<&str> = l.split(": ").collect();
                let input = match *parts[1].split(' ').collect::<Vec<_>>().as_slice() {
                    [val] => Input::Number {
                        val: val.parse().unwrap(),
                    },
                    [left, op, right] => Input::Op { left, right, op },
                    _ => panic!("Unexpected input: {}", parts[1]),
                };
                (parts[0], input)
            })
            .collect()
    }
}

fn apply_op(op: &str, left: i64, right: i64) -> i64 {
    match op {
        "+" => left + right,
        "-" => left - right,
        "*" => left * right,
        "/" => left / right,
        unknown => panic!("Unknown operation: {unknown}"),
    }
}

fn unapply_left_op(op: &str, left: i64, target: i64) -> i64 {
    match op {
        "+" => target - left,
        "-" => left - target,
        "*" => target / left,
        "/" => left / target,
        unknown => panic!("Unknown operation: {unknown}"),
    }
}

fn unapply_right_op(op: &str, right: i64, target: i64) -> i64 {
    match op {
        "+" => target - right,
        "-" => target + right,
        "*" => target / right,
        "/" => target * right,
        unknown => panic!("Unknown operation: {unknown}"),
    }
}

#[aoc(day21, part1)]
pub fn part1(input: &str) -> i64 {
    let input = Input::parse(input);
    let mut numbers: HashMap<&str, i64> = HashMap::new();
    let mut stack: Vec<&str> = Vec::new();
    stack.push("root");

    while let Some(name) = stack.last() {
        match *input.get(name).unwrap() {
            Input::Number { val } => {
                numbers.insert(name, val);
                stack.pop();
            }
            Input::Op { left, right, op } => match (numbers.get(left), numbers.get(right)) {
                (Some(left_val), Some(right_val)) => {
                    numbers.insert(name, apply_op(op, *left_val, *right_val));
                    stack.pop();
                }
                (None, _) => {
                    stack.push(left);
                }
                (_, None) => {
                    stack.push(right);
                }
            },
        }
    }

    *numbers.get("root").unwrap()
}

#[aoc(day21, part2)]
pub fn part2(input: &str) -> i64 {
    let input = Input::parse(input);
    let mut numbers: HashMap<&str, i64> = HashMap::new();
    let mut path: Option<Vec<&str>> = None;
    let mut stack: Vec<&str> = Vec::new();
    stack.push("root");

    while let Some(name) = stack.last() {
        if *name == "humn" {
            path = Some(stack.clone());
        }
        match *input.get(name).unwrap() {
            Input::Number { val } => {
                numbers.insert(name, val);
                stack.pop();
            }
            Input::Op { left, right, op } => match (numbers.get(left), numbers.get(right)) {
                (Some(left_val), Some(right_val)) => {
                    numbers.insert(*name, apply_op(op, *left_val, *right_val));
                    stack.pop();
                }
                (None, _) => {
                    stack.push(left);
                }
                (_, None) => {
                    stack.push(right);
                }
            },
        }
    }
    let mut target = 0;
    for (name, next) in path.unwrap().into_iter().tuple_windows() {
        target = match *input.get(name).unwrap() {
            Input::Op { left, right, .. } if name == "root" => {
                if left == next {
                    *numbers.get(right).unwrap()
                } else if right == next {
                    *numbers.get(left).unwrap()
                } else {
                    panic!("No matching op: {name} -> {next}")
                }
            }
            Input::Op { left, right, op } => {
                if left == next {
                    unapply_right_op(op, *numbers.get(right).unwrap(), target)
                } else if right == next {
                    unapply_left_op(op, *numbers.get(left).unwrap(), target)
                } else {
                    panic!("No matching op: {name} -> {next}")
                }
            }
            _ => panic!("Op error: {name}"),
        };
    }
    target
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let example = include_str!("examples/day21.txt");
        assert_eq!(part1(example), 152);
    }

    #[test]
    fn test_example_part2() {
        let example = include_str!("examples/day21.txt");
        assert_eq!(part2(example), 301);
    }
}
