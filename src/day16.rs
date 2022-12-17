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
    // All nodes as parsed from input.
    nodes: Vec<Node>,
    // Distance from each node to every other node.
    dist: Vec<Vec<usize>>,
    // Index of the start node.
    start: usize,
    // Pairs (index, pressure) for valves that have pressure >.
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
    // Map each node name to index, where index is the index of the input line.
    let map: HashMap<&str, usize> = nodes
        .iter()
        .enumerate()
        .map(|(i, s)| (s.name.as_str(), i))
        .collect();
    let n = nodes.len();
    // Start node is "AA".
    let start = nodes
        .iter()
        .enumerate()
        .find(|(_i, n)| n.name == "AA")
        .unwrap()
        .0;
    // We can consider jumping only between valves with pressure to speed up backtracking.
    // Lets calculuate distance between each nodes with Floyd–Warshall algo.
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
    // Take only nodes with pressure > 0 as candidates for backtracking.
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

// Simple backtracking over all candidates order until we are out of budget.
// Can be potentially improved by some pruning, but takes only ~11 ms for Part 1, so not worth it.
// 'Greedy' (e.g. taking the best option on every step) doesn't work.
// Likely there is some DP approach, but it is not apparent for me.
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

// Stupid backtracking with 2 pointers.
// For each unvisited point we choose the one, that leaves the biggest budget.
// This seems to work for test and my input, but I'm not convinced this is algorithmically correct.
// This is very slow - ~40s on my laptop. Likely can be optimized with some pruning,
// but ideally I should look for some algo approach.
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

// Bottom-up DP for part 1. Apparently much slower than backtracking.
#[aoc(day16, part1, dp)]
pub fn part1_dp(input: &str) -> usize {
    let input = parse_input(input);
    let n = input.candidates.len();
    let lim: usize = 1 << n;
    let budget = 30;
    let mut dp = vec![vec![vec![0_usize; n + 1]; lim]; budget + 1];

    let is_bit_set = |m, u| (m & (1_usize << u)) != 0;
    let set_bit = |m, u| m | (1_usize << u);

    for b in 2..=budget {
        for m in 0..lim {
            for (i, &(u, _)) in input
                .candidates
                .iter()
                .enumerate()
                // Add start ('AA') as one of the candidates.
                .chain(std::iter::once((n, &(input.start, 0_usize))))
            {
                let mut r = 0;
                for (j, &(v, p)) in input.candidates.iter().enumerate() {
                    let d = input.dist[u][v];
                    if !is_bit_set(m, j) && b > d {
                        r = r.max(dp[b - d - 1][set_bit(m, j)][j] + p * (b - d - 1));
                    }
                }
                dp[b][m][i] = r;
            }
        }
    }

    dp[budget][0][n]
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

// Can't figure out the right bottom-up DP for the part2. Also very slow (2x faster than backtracking)
// WIP for now.
//#[aoc(day16, part2, dp)]
#[allow(unused)]
pub fn part2_dp(input: &str) -> usize {
    let input = parse_input(input);
    let n = input.candidates.len();
    let lim: usize = 1 << n;
    let budget = 26;
    let mut dp = vec![vec![vec![vec![0_usize; n + 1]; lim]; budget + 1]; budget + 1];

    let is_bit_set = |m, u| (m & (1_usize << u)) != 0;
    let set_bit = |m, u| m | (1_usize << u);

    for b1 in 2..=budget {
        for b2 in 2..=b1 {
            for m in 0..lim {
                for (i, &(u, _)) in input
                    .candidates
                    .iter()
                    .enumerate()
                    // Add start ('AA') as one of the candidates.
                    .chain(std::iter::once((n, &(input.start, 0_usize))))
                {
                    let mut r = 0;
                    for (j, &(v, p)) in input.candidates.iter().enumerate() {
                        let d = input.dist[u][v];
                        if !is_bit_set(m, j) {
                            if b1 > d {
                                r = r.max(dp[b1 - d - 1][b2][set_bit(m, j)][j] + p * (b1 - d - 1));
                            }
                            if b2 > d {
                                r = r.max(dp[b1][b2 - d - 1][set_bit(m, j)][j] + p * (b2 - d - 1));
                            }
                        }
                    }
                    dp[b1][b2][m][i] = r;
                }
            }
        }
    }

    dp[budget][budget][0][n]
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
    fn test_example_part1_dp() {
        let example = include_str!("examples/day16.txt");
        assert_eq!(part1_dp(example), 1651);
    }

    #[test]
    fn test_example_part2() {
        let example = include_str!("examples/day16.txt");
        assert_eq!(part2(example), 1707);
    }

    /*
    #[test]
    fn test_example_part2_dp() {
        let example = include_str!("examples/day16.txt");
        assert_eq!(part2_dp(example), 1707);
    }
    */
}
