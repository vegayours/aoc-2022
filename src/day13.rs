use std::{cmp::Ordering, fmt::Debug};

use aoc_runner_derive::aoc;

#[derive(PartialEq, Eq, Clone)]
enum Node {
    Integer(i32),
    List(Vec<Node>),
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(arg0) => arg0.fmt(f),
            Self::List(arg0) => arg0.fmt(f),
        }
    }
}

fn parse_node(input: &str) -> Node {
    let mut stack: Vec<Vec<Node>> = Vec::new();
    let mut number: Option<i32> = None;
    for b in input.as_bytes().iter() {
        match *b {
            b'[' => stack.push(Vec::new()),
            b']' => {
                if let Some(n) = number {
                    stack.last_mut().unwrap().push(Node::Integer(n));
                    number = None;
                }
                let v = stack.pop().unwrap();
                if let Some(parent) = stack.last_mut() {
                    parent.push(Node::List(v));
                } else {
                    stack.push(v);
                }
            }
            b',' => {
                if let Some(n) = number {
                    stack.last_mut().unwrap().push(Node::Integer(n));
                    number = None;
                }
            }
            x if (b'0'..=b'9').contains(&x) => {
                let digit = (x - b'0') as i32;
                number = match number {
                    Some(v) => Some(v * 10 + digit),
                    None => Some(digit),
                };
            }
            _ => panic!("Wrong input"),
        }
    }
    assert!(stack.len() == 1);
    Node::List(stack.pop().unwrap())
}

struct Task {
    left: Node,
    right: Node,
}

impl Task {
    fn parse(input: &str) -> Task {
        let mut it = input.lines().map(parse_node);
        Task {
            left: it.next().unwrap(),
            right: it.next().unwrap(),
        }
    }
}

fn compare_nodes(l: &Node, r: &Node) -> Ordering {
    compare(std::iter::once(l), std::iter::once(r))
}

fn compare<'a>(
    mut left_it: impl Iterator<Item = &'a Node>,
    mut right_it: impl Iterator<Item = &'a Node>,
) -> Ordering {
    loop {
        let order = match (left_it.next(), right_it.next()) {
            (None, None) => break Ordering::Equal,
            (Some(_), None) => Ordering::Greater,
            (None, Some(_)) => Ordering::Less,
            (Some(l_node), Some(r_node)) => match (l_node, r_node) {
                (Node::Integer(l), Node::Integer(r)) => l.cmp(r),
                (Node::List(l), Node::List(r)) => compare(l.iter(), r.iter()),
                (Node::List(l), r) => compare(l.iter(), std::iter::once(r)),
                (l, Node::List(r)) => compare(std::iter::once(l), r.iter()),
            },
        };
        if order != Ordering::Equal {
            break order;
        }
    }
}

#[aoc(day13, part1)]
pub fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(Task::parse)
        .map(|t| compare_nodes(&t.left, &t.right))
        .enumerate()
        .filter(|(_i, o)| *o == Ordering::Less)
        .map(|(i, _o)| i + 1)
        .sum()
}

fn make_divider(value: i32) -> Node {
    Node::List(vec![Node::List(vec![Node::Integer(value)])])
}

#[aoc(day13, part2)]
pub fn part2(input: &str) -> usize {
    let nodes = input.lines().filter(|l| !l.is_empty()).map(parse_node);
    let div1 = make_divider(2);
    let div2 = make_divider(6);

    let mut count1 = 1_usize;
    let mut count2 = 2_usize;

    for node in nodes {
        if compare_nodes(&node, &div1) == Ordering::Less {
            count1 += 1;
            count2 += 1;
        } else if compare_nodes(&node, &div2) == Ordering::Less {
            count2 += 1;
        }
    }
    count1 * count2
}

#[aoc(day13, part2, sorted)]
pub fn part2_sorted(input: &str) -> usize {
    let mut nodes: Vec<Node> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(parse_node)
        .collect();

    let div1 = make_divider(2);
    let div2 = make_divider(6);

    nodes.push(div1.clone());
    nodes.push(div2.clone());
    nodes.sort_unstable_by(compare_nodes);
    nodes
        .iter()
        .enumerate()
        .filter(|(_i, v)| *v == &div1 || *v == &div2)
        .map(|(i, _v)| i + 1)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let example = include_str!("examples/day13.txt");
        assert_eq!(part1(example), 13);
    }

    #[test]
    fn test_example_part2() {
        let example = include_str!("examples/day13.txt");
        assert_eq!(part2(example), 140);
    }

    #[test]
    fn test_example_part2_sorted() {
        let example = include_str!("examples/day13.txt");
        assert_eq!(part2_sorted(example), 140);
    }
}
