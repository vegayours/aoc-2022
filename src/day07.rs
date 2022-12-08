use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;

use aoc_runner_derive::aoc;

#[derive(Debug)]
enum Node {
    File(usize),
    Directory(Rc<RefCell<HashMap<String, Node>>>),
}

fn parse(input: &str) -> Node {
    let root = Rc::new(RefCell::new(HashMap::new()));
    let mut stack: Vec<Rc<RefCell<HashMap<String, Node>>>> = Vec::new();
    stack.push(root.clone());

    for line in input.lines().map(str::trim) {
        let parts: Vec<&str> = line.split(' ').collect();
        match *parts.as_slice() {
            ["$", "ls"] | ["dir", _] => {}
            [size, file_name] => {
                stack
                    .last()
                    .unwrap()
                    .borrow_mut()
                    .insert(String::from(file_name), Node::File(size.parse().unwrap()));
            }
            ["$", "cd", path] => match path {
                "/" => {
                    stack.truncate(1);
                }
                ".." => {
                    stack.pop();
                }
                _ => {
                    let dir = Rc::new(RefCell::new(HashMap::new()));
                    stack
                        .last()
                        .unwrap()
                        .borrow_mut()
                        .insert(String::from(path), Node::Directory(dir.clone()));
                    stack.push(dir);
                }
            },
            _ => panic!("Unexpected line: {}", line),
        }
    }
    Node::Directory(root)
}

fn walk(node: &Node, dirs: &mut Vec<usize>) -> usize {
    match node {
        Node::File(size) => *size,
        Node::Directory(dir) => {
            let cur = dir.borrow().values().map(|child| walk(child, dirs)).sum();
            dirs.push(cur);
            cur
        }
    }
}

#[aoc(day07, part1)]
pub fn part1(input: &str) -> usize {
    let node = parse(input);
    let mut dirs: Vec<usize> = Vec::new();
    walk(&node, &mut dirs);
    dirs.into_iter().filter(|d| *d <= 100000).sum()
}

#[aoc(day07, part2)]
pub fn part2(input: &str) -> usize {
    let node = parse(input);
    let mut dirs: Vec<usize> = Vec::new();
    let mut total = walk(&node, &mut dirs);
    total = std::cmp::min(total, 70_000_000);
    if total <= 40_000_000 {
        0
    } else {
        let min_target: usize = total - 40_000_000;
        dirs.into_iter().filter(|x| *x >= min_target).min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let example = include_str!("examples/day07.txt");
        assert_eq!(part1(example), 95437);
    }

    #[test]
    fn test_example_part2() {
        let example = include_str!("examples/day07.txt");
        assert_eq!(part2(example), 24933642);
    }
}
