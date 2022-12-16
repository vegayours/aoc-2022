use std::{cmp::Reverse, collections::HashMap};

use aoc_runner_derive::aoc;
use regex::Regex;

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.+)").unwrap();
}

#[derive(Debug)]
struct Node {
    name: String,
    pressure: usize,
    neighbors: Vec<String>,
}

#[derive(Debug)]
struct Input {
    nodes: Vec<Node>,
    dist: Vec<Vec<usize>>,
    start: usize,
    candidates: Vec<(usize, usize)>,
}

fn parse_node(input: &str) -> Node {
    let captures = RE.captures(input).unwrap();
    Node {
        name: captures.get(1).unwrap().as_str().into(),
        pressure: captures.get(2).unwrap().as_str().parse().unwrap(),
        neighbors: captures
            .get(3)
            .unwrap()
            .as_str()
            .split(", ")
            .map(|x| x.to_string())
            .collect(),
    }
}

fn parse_input(input: &str) -> Input {
    let nodes: Vec<Node> = input.lines().map(parse_node).collect();
    let map: HashMap<&str, usize> = nodes
        .iter()
        .enumerate()
        .map(|(i, s)| (s.name.as_str(), i))
        .collect();
    let n = nodes.len();
    let start = nodes
        .iter()
        .enumerate()
        .find(|(_i, n)| n.name == "AA")
        .unwrap()
        .0;
    let mut dist = vec![vec![usize::MAX / 2; n]; n];
    for (u, node) in nodes.iter().enumerate() {
        dist[u][u] = 0;
        for n in &node.neighbors {
            dist[u][*map.get(n.as_str()).unwrap()] = 1;
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                let d = dist[i][k] + dist[k][j];
                if d < dist[i][j] {
                    dist[i][j] = d;
                }
            }
        }
    }

    let candidates: Vec<(usize, usize)> = nodes
        .iter()
        .enumerate()
        .filter(|(_i, n)| n.pressure > 0)
        .map(|(i, n)| (i, n.pressure))
        .collect();
    Input {
        nodes,
        dist,
        start,
        candidates,
    }
}

fn backtrack1(
    (u, budget): (usize, usize),
    pressure: usize,
    dist: &Vec<Vec<usize>>,
    nodes: &[(usize, usize)],
    visited: &mut [bool],
) -> usize {
    if budget < 2 {
        return pressure;
    }
    let mut max_pressure = pressure;
    for &(v, v_pressure) in nodes {
        if visited[v] || budget <= dist[u][v] + 1 {
            continue;
        }
        visited[v] = true;
        let next_budget = budget - dist[u][v] - 1;
        let next_pressure = backtrack1(
            (v, next_budget),
            pressure + v_pressure * next_budget,
            dist,
            nodes,
            visited,
        );
        if next_pressure > max_pressure {
            max_pressure = next_pressure;
        }
        visited[v] = false;
    }
    max_pressure
}

fn backtrack2(
    (u_1, budget_1): (usize, usize),
    (u_2, budget_2): (usize, usize),
    pressure: usize,
    dist: &Vec<Vec<usize>>,
    nodes: &[(usize, usize)],
    visited: &mut [bool],
) -> usize {
    if budget_1 < 2 {
        return backtrack1((u_2, budget_2), pressure, dist, nodes, visited);
    }
    if budget_2 < 2 {
        return backtrack1((u_1, budget_1), pressure, dist, nodes, visited);
    }
    let mut max_pressure = pressure;
    for &(v, v_pressure) in nodes {
        if !visited[v] {
            visited[v] = true;

            if budget_1 > dist[u_1][v] + 1
                && (budget_2 < dist[u_2][v] + 1
                    || budget_1 - dist[u_1][v] >= budget_2 - dist[u_2][v])
            {
                let next_budget_1 = budget_1 - dist[u_1][v] - 1;
                max_pressure = max_pressure.max(backtrack2(
                    (v, next_budget_1),
                    (u_2, budget_2),
                    pressure + v_pressure * next_budget_1,
                    dist,
                    nodes,
                    visited,
                ));
            } else if budget_2 > dist[u_2][v] + 1 {
                let next_budget_2 = budget_2 - dist[u_2][v] - 1;
                max_pressure = max_pressure.max(backtrack2(
                    (u_1, budget_1),
                    (v, next_budget_2),
                    pressure + v_pressure * next_budget_2,
                    dist,
                    nodes,
                    visited,
                ));
            }

            visited[v] = false;
        }
    }
    max_pressure
}

#[aoc(day16, part1)]
pub fn part1(input: &str) -> usize {
    let input = parse_input(input);
    let mut candidates = input.candidates;
    candidates.sort_by_key(|(i, p)| (Reverse(*p), input.dist[0][*i]));
    let mut visited = vec![false; input.nodes.len()];

    backtrack1((input.start, 30), 0, &input.dist, &candidates, &mut visited)
}

#[aoc(day16, part2)]
pub fn part2(input: &str) -> usize {
    let input = parse_input(input);
    let mut candidates = input.candidates;
    candidates.sort_by_key(|(i, p)| (Reverse(*p), input.dist[0][*i]));
    let mut visited = vec![false; input.nodes.len()];
    backtrack2(
        (input.start, 26),
        (input.start, 26),
        0,
        &input.dist,
        &candidates,
        &mut visited,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let example = include_str!("examples/day16.txt");
        assert_eq!(part1(example), 1651);
    }

    #[test]
    fn test_example_part2() {
        let example = include_str!("examples/day16.txt");
        assert_eq!(part2(example), 1707);
    }
}
