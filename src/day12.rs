use std::collections::VecDeque;

use aoc_runner_derive::aoc;

fn height(cell: u8) -> usize {
    let x = match cell {
        b'S' => 0,
        b'E' => b'z' - b'a',
        c => c - b'a',
    };
    x as usize
}

fn find_cell(grid: &[&[u8]], cell: u8) -> (usize, usize) {
    grid.iter()
        .enumerate()
        .flat_map(|(i, c)| c.iter().enumerate().map(move |(j, c)| ((i, j), c)))
        .find(|(_, c)| **c == cell)
        .unwrap()
        .0
}

fn neighbors((i, j): (usize, usize), m: usize, n: usize) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    if i > 0 {
        res.push((i - 1, j));
    }
    if j > 0 {
        res.push((i, j - 1));
    }
    if i + 1 < m {
        res.push((i + 1, j));
    }
    if j + 1 < n {
        res.push((i, j + 1));
    }
    res
}

fn solve(grid: &[&[u8]]) -> Vec<Vec<usize>> {
    let m = grid.len();
    let n = grid[0].len();
    let end = find_cell(grid, b'E');
    let mut visited = vec![vec![usize::MAX; n]; m];
    let mut min_heap = VecDeque::new();
    visited[end.0][end.1] = 0;
    min_heap.push_back(end);

    while let Some((i, j)) = min_heap.pop_front() {
        if grid[i][j] == b'S' || grid[i][j] == b'a' {
            continue;
        }
        for (x, y) in neighbors((i, j), m, n) {
            if height(grid[i][j]) > height(grid[x][y]) + 1 {
                continue;
            }
            if visited[x][y] == usize::MAX {
                visited[x][y] = visited[i][j] + 1;
                min_heap.push_back((x, y));
            }
        }
    }

    visited
}

#[aoc(day12, part1)]
pub fn part1(input: &str) -> usize {
    let grid: Vec<&[u8]> = input.lines().map(|l| l.trim().as_bytes()).collect();
    let start = find_cell(&grid, b'S');
    let visited = solve(&grid);
    visited[start.0][start.1]
}

#[aoc(day12, part2)]
pub fn part2(input: &str) -> usize {
    let grid: Vec<&[u8]> = input.lines().map(|l| l.trim().as_bytes()).collect();
    let visited = solve(&grid);
    grid.iter()
        .zip(visited.iter())
        .flat_map(|(grid_row, visited_row)| grid_row.iter().zip(visited_row.iter()))
        .filter(|(g, _)| **g == b'S' || **g == b'a')
        .map(|(_, v)| *v)
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let example = include_str!("examples/day12.txt");
        assert_eq!(part1(example), 31);
    }

    #[test]
    fn test_example_part2() {
        let example = include_str!("examples/day12.txt");
        assert_eq!(part2(example), 29);
    }
}
