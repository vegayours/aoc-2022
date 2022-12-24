use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use aoc_runner_derive::aoc;

struct Grid<'a> {
    data: Vec<&'a [u8]>,
}

impl<'a> Grid<'a> {
    fn parse(input: &'a str) -> Grid<'a> {
        Grid {
            data: input.lines().map(|l| l.as_bytes()).collect(),
        }
    }

    fn is_allowed(&self, (x, y): (i32, i32), epoch: i32) -> bool {
        let rows = (self.data.len() - 2) as i32;
        let cols = (self.data[0].len() - 2) as i32;

        if x < 0 || x > rows + 1 {
            return false;
        }

        [
            (x, y, b'#'),
            (((x - 1) + epoch) % rows + 1, y, b'^'),
            ((rows + (x - 1) - epoch % rows) % rows + 1, y, b'v'),
            (x, (cols + (y - 1) - epoch % cols) % cols + 1, b'>'),
            (x, ((y - 1) + epoch) % cols + 1, b'<'),
        ]
        .iter()
        .all(|&(x, y, d)| self.data[x as usize][y as usize] != d)
    }
}

fn candidates((x, y): (i32, i32)) -> [(i32, i32); 5] {
    [(x, y), (x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
}

fn distance((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn solve(start: (i32, i32), end: (i32, i32), start_epoch: i32, grid: &Grid) -> i32 {
    let mut visited = HashSet::new();
    visited.insert((start, start_epoch));

    let mut pq = BinaryHeap::new();
    pq.push((
        Reverse(distance(start, end) + start_epoch),
        Reverse(start_epoch),
        start,
    ));

    while let Some((_, Reverse(epoch), point)) = pq.pop() {
        if point == end {
            return epoch;
        }
        for next in candidates(point) {
            if grid.is_allowed(next, epoch + 1) && !visited.contains(&(next, epoch + 1)) {
                visited.insert((next, epoch + 1));
                pq.push((
                    Reverse(distance(next, end) + epoch + 1),
                    Reverse(epoch + 1),
                    next,
                ));
            }
        }
    }
    panic!("No solution");
}

#[aoc(day24, part1)]
pub fn part1(input: &str) -> i32 {
    let grid = Grid::parse(input);
    let end = (
        (grid.data.len() - 1) as i32,
        (grid.data[1].len() - 2) as i32,
    );
    let start = (0, 1);
    solve(start, end, 0, &grid)
}

#[aoc(day24, part2)]
pub fn part2(input: &str) -> i32 {
    let grid = Grid::parse(input);
    let end = (
        (grid.data.len() - 1) as i32,
        (grid.data[1].len() - 2) as i32,
    );
    let start = (0, 1);
    let first = solve(start, end, 0, &grid);
    let second = solve(end, start, first, &grid);
    solve(start, end, second, &grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let example = include_str!("examples/day24.txt");
        assert_eq!(part1(example), 18);
    }

    #[test]
    fn test_example_part2() {
        let example = include_str!("examples/day24.txt");
        assert_eq!(part2(example), 54);
    }
}
