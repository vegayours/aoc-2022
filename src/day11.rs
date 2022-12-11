use std::{cmp::Reverse, collections::VecDeque};

use aoc_runner_derive::aoc;

struct Monkey {
    items: VecDeque<u64>,
    inspect: Box<dyn Fn(u64) -> u64>,
    test_div: u64,
    test: Box<dyn Fn(u64) -> usize>,
    total_inspected: usize,
}

fn parse_expr(expr: &str) -> Box<dyn Fn(u64) -> u64> {
    match *expr
        .split_ascii_whitespace()
        .collect::<Vec<&str>>()
        .as_slice()
    {
        ["old", "+", "old"] => Box::new(|x| x.wrapping_add(x)),
        ["old", "*", "old"] => Box::new(|x| x.wrapping_mul(x)),
        ["old", "+", val] => {
            let val: u64 = val.parse().unwrap();
            Box::new(move |x| x.wrapping_add(val))
        }
        ["old", "*", val] => {
            let val: u64 = val.parse().unwrap();
            Box::new(move |x| x.wrapping_mul(val))
        }
        _ => panic!("Unsupported expression: {expr}"),
    }
}

fn parse_value<'a>(it: &mut impl Iterator<Item = &'a str>, separator: &'a str) -> &'a str {
    it.next().unwrap().split(separator).nth(1).unwrap()
}

fn parse_monkey(index: usize, input: &str) -> Monkey {
    let mut it = input.lines().skip(1).map(str::trim);
    let items: VecDeque<u64> = parse_value(&mut it, ": ")
        .split(", ")
        .flat_map(|x| x.parse())
        .collect();
    let inspect_expr = parse_value(&mut it, " = ");
    let test_div: u64 = parse_value(&mut it, "by ").parse().unwrap();
    let true_monkey: usize = parse_value(&mut it, "monkey ").parse().unwrap();
    let false_monkey: usize = parse_value(&mut it, "monkey ").parse().unwrap();
    assert_ne!(index, true_monkey, "True monkey can't be self");
    assert_ne!(index, false_monkey, "False monkey can't be self");
    Monkey {
        items,
        inspect: parse_expr(inspect_expr),
        test_div,
        test: Box::new(move |x| {
            if x % test_div == 0 {
                true_monkey
            } else {
                false_monkey
            }
        }),
        total_inspected: 0,
    }
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .enumerate()
        .map(|(i, input)| parse_monkey(i, input))
        .collect()
}

impl Monkey {
    fn inspect(&mut self, adjust: &impl Fn(u64) -> u64) -> Option<(usize, u64)> {
        self.items.pop_front().map(|item| {
            self.total_inspected += 1;
            let next_item = (self.inspect)(item);
            let next_item = adjust(next_item);
            let next_monkey = (self.test)(next_item);
            (next_monkey, next_item)
        })
    }

    fn add(&mut self, item: u64) {
        self.items.push_back(item);
    }
}

fn solve(monkeys: &mut [Monkey], rounds: usize, adjust: impl Fn(u64) -> u64) -> usize {
    for _round in 0..rounds {
        for i in 0..monkeys.len() {
            while let Some((next_monkey, next_item)) = monkeys[i].inspect(&adjust) {
                monkeys[next_monkey].add(next_item);
            }
        }
    }
    let mut inspected: Vec<usize> = monkeys.iter().map(|m| m.total_inspected).collect();
    inspected.sort_unstable_by_key(|x| Reverse(*x));
    inspected[0] * inspected[1]
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> usize {
    let mut monkeys = parse_monkeys(input);
    solve(&mut monkeys, 20, |x| x / 3)
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> usize {
    let mut monkeys = parse_monkeys(input);
    let modulo = monkeys.iter().map(|m| m.test_div).product::<u64>();
    solve(&mut monkeys, 10000, |x| x % modulo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let example = include_str!("examples/day11.txt");
        assert_eq!(part1(example), 10605);
    }

    #[test]
    fn test_example_part2() {
        let example = include_str!("examples/day11.txt");
        assert_eq!(part2(example), 2713310158);
    }
}
