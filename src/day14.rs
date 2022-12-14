use std::fmt::{Display, Write};

use aoc_runner_derive::aoc;

use itertools::Itertools;

type Coord = (usize, usize);
type Path = Vec<Coord>;

fn parse_path(input: &str) -> Path {
    input
        .split(" -> ")
        .map(|x| {
            let (y, x) = x
                .split(',')
                .flat_map(|part| part.parse::<usize>())
                .next_tuple::<Coord>()
                .unwrap();
            (x, y)
        })
        .collect()
}

fn parse_input(input: &str) -> Vec<Path> {
    input.lines().map(parse_path).collect()
}

fn parse_limits(paths: &[Path]) -> (Coord, Coord) {
    let mut min_x = usize::MAX;
    let mut max_x = usize::MIN;
    let mut min_y = usize::MAX;
    let mut max_y = usize::MIN;
    for (x, y) in paths.iter().flat_map(|path| path.iter()) {
        min_x = min_x.min(*x);
        max_x = max_x.max(*x);
        min_y = min_y.min(*y);
        max_y = max_y.max(*y);
    }
    ((min_x, min_y), (max_x, max_y))
}

struct Grid {
    rows: usize,
    cols: usize,
    start: (usize, usize),
    data: Vec<Vec<u8>>,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rows = self
            .data
            .iter()
            .map(|r| std::str::from_utf8(r.as_slice()).unwrap());
        for (i, row) in rows.enumerate() {
            f.write_str(row)?;
            if i + 1 < self.rows {
                f.write_char('\n')?;
            }
        }
        Ok(())
    }
}

#[derive(PartialEq, Eq)]
enum AdvanceOutcome {
    Locked,
    InAbyss,
    StartBlocked,
}

impl Grid {
    fn from_paths(paths: &[Path]) -> Grid {
        let ((_min_x, min_y), (max_x, max_y)) = parse_limits(paths);
        let rows = max_x + 1;
        let cols = max_y - min_y + 2 * max_x;
        let translate_y = |x| x + max_x - min_y;
        let start = (0, translate_y(500));
        let mut data = vec![vec![b'.'; cols]; rows];
        for path in paths {
            let coords = path.iter().map(|(x, y)| (x, translate_y(*y)));
            for ((from_x, from_y), (to_x, to_y)) in coords.tuple_windows() {
                if from_x == to_x {
                    for y in from_y.min(to_y)..=from_y.max(to_y) {
                        data[*from_x][y] = b'#';
                    }
                } else if from_y == to_y {
                    #[allow(clippy::needless_range_loop)]
                    for x in *from_x.min(to_x)..=*from_x.max(to_x) {
                        data[x][from_y] = b'#';
                    }
                } else {
                    panic!("Wrong path: {:?}", ((from_x, from_y), (to_x, to_y)))
                }
            }
        }
        data[start.0][start.1] = b'+';
        Grid {
            rows,
            cols,
            start,
            data,
        }
    }

    fn add_floor(&mut self) {
        self.data.push(vec![b'.'; self.cols]);
        self.data.push(vec![b'#'; self.cols]);
        self.rows += 2;
    }

    fn lock_next(&mut self) -> AdvanceOutcome {
        let (mut x, mut y) = self.start;
        loop {
            if x + 1 == self.rows {
                break AdvanceOutcome::InAbyss;
            }
            let next_points = [(x + 1, y), (x + 1, y.saturating_sub(1)), (x + 1, y + 1)];
            if let Some(next) = next_points
                .iter()
                .find(|(x, y)| *y < self.cols && self.data[*x][*y] == b'.')
            {
                (x, y) = *next;
            } else {
                if (x, y) != self.start {
                    self.data[x][y] = b'o';
                    break AdvanceOutcome::Locked;
                }
                break AdvanceOutcome::StartBlocked;
            }
        }
    }
}

#[aoc(day14, part1)]
pub fn part1(input: &str) -> usize {
    let paths = parse_input(input);
    let mut grid = Grid::from_paths(&paths);
    let mut i = 0;
    while grid.lock_next() != AdvanceOutcome::InAbyss {
        i += 1;
    }
    i
}

#[aoc(day14, part2)]
pub fn part2(input: &str) -> usize {
    let paths = parse_input(input);
    let mut grid = Grid::from_paths(&paths);
    grid.add_floor();
    let mut i = 0;
    while grid.lock_next() != AdvanceOutcome::StartBlocked {
        i += 1;
    }
    i + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let example = include_str!("examples/day14.txt");
        assert_eq!(part1(example), 24);
    }

    #[test]
    fn test_example_part2() {
        let example = include_str!("examples/day14.txt");
        assert_eq!(part2(example), 93);
    }
}
