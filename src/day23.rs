use std::collections::{HashMap, HashSet};

use aoc_runner_derive::aoc;

fn parse_input(input: &str) -> HashSet<(i32, i32)> {
    let mut occupied: HashSet<(i32, i32)> = HashSet::new();
    for (i, row) in input.lines().enumerate() {
        for (j, val) in row.as_bytes().iter().enumerate() {
            if *val == b'#' {
                occupied.insert((i as i32, j as i32));
            }
        }
    }
    occupied
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    North,
    South,
    West,
    East,
}

fn get_directions(step: usize) -> Vec<Direction> {
    [
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ]
    .into_iter()
    .cycle()
    .skip(step % 4)
    .take(4)
    .collect()
}

impl Direction {
    fn can_move(&self, (x, y): (i32, i32), occupied: &HashSet<(i32, i32)>) -> bool {
        let shifts = match self {
            Direction::North => [(-1, -1), (-1, 0), (-1, 1)],
            Direction::South => [(1, -1), (1, 0), (1, 1)],
            Direction::West => [(-1, -1), (0, -1), (1, -1)],
            Direction::East => [(-1, 1), (0, 1), (1, 1)],
        };
        shifts
            .iter()
            .map(|(dx, dy)| (x + dx, y + dy))
            .all(|coord| !occupied.contains(&coord))
    }

    fn next_point(&self, (x, y): (i32, i32)) -> (i32, i32) {
        match self {
            Direction::North => (x - 1, y),
            Direction::South => (x + 1, y),
            Direction::West => (x, y - 1),
            Direction::East => (x, y + 1),
        }
    }
}

struct Bounds {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl Bounds {
    fn for_iter<'a>(iter: impl Iterator<Item = &'a (i32, i32)>) -> Self {
        let mut bounds = Self::new();
        for &point in iter {
            bounds.add_point(point);
        }
        bounds
    }

    fn new() -> Self {
        Self {
            min_x: i32::MAX,
            max_x: i32::MIN,
            min_y: i32::MAX,
            max_y: i32::MIN,
        }
    }

    fn add_point(&mut self, (x, y): (i32, i32)) {
        self.min_x = self.min_x.min(x);
        self.max_x = self.max_x.max(x);
        self.min_y = self.min_y.min(y);
        self.max_y = self.max_y.max(y);
    }

    fn x_len(&self) -> i32 {
        self.max_x - self.min_x + 1
    }

    fn y_len(&self) -> i32 {
        self.max_y - self.min_y + 1
    }
}

fn simulate_step(step: usize, occupied: &HashSet<(i32, i32)>) -> (bool, HashSet<(i32, i32)>) {
    let mut moved = false;
    let dirs = get_directions(step);
    let mut next: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
    for &point in occupied {
        let (next_point, cnt) = dirs
            .iter()
            .flat_map(|dir| {
                if dir.can_move(point, occupied) {
                    Some(dir.next_point(point))
                } else {
                    None
                }
            })
            .fold((None, 0), |(first, cnt), point| {
                (first.or(Some(point)), cnt + 1)
            });
        let next_point = if cnt == 4 {
            point
        } else {
            next_point.unwrap_or(point)
        };
        next.entry(next_point)
            .and_modify(|e| e.push(point))
            .or_insert_with(|| vec![point]);
    }
    let next_occupied = next
        .into_iter()
        .flat_map(|(point, origins)| {
            if origins.len() == 1 {
                if point != origins[0] {
                    moved = true;
                }
                vec![point].into_iter()
            } else {
                origins.into_iter()
            }
        })
        .collect();
    (moved, next_occupied)
}

#[aoc(day23, part1)]
pub fn part1(input: &str) -> i32 {
    let mut occupied = parse_input(input);
    for step in 0..10 {
        (_, occupied) = simulate_step(step, &occupied);
    }
    let bounds = Bounds::for_iter(occupied.iter());
    bounds.x_len() * bounds.y_len() - occupied.len() as i32
}

#[aoc(day23, part2)]
pub fn part2(input: &str) -> usize {
    let mut occupied = parse_input(input);
    for step in 0.. {
        let (changed, next_occupied) = simulate_step(step, &occupied);
        if !changed {
            return step + 1;
        }
        occupied = next_occupied;
    }
    panic!("No solution");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let example = include_str!("examples/day23.txt");
        assert_eq!(part1(example), 110);
    }

    #[test]
    fn test_example_part2() {
        let example = include_str!("examples/day23.txt");
        assert_eq!(part2(example), 20);
    }
}
