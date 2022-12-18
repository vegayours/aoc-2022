use std::{
    collections::{HashSet, VecDeque},
    ops::RangeInclusive,
};

use aoc_runner_derive::aoc;

type Cube = (i32, i32, i32);

fn parse_input(input: &str) -> Vec<Cube> {
    input
        .lines()
        .map(|l| {
            let mut coords = l.split(',').flat_map(|x| x.parse());
            (
                coords.next().unwrap(),
                coords.next().unwrap(),
                coords.next().unwrap(),
            )
        })
        .collect()
}

fn neighbors(&(x, y, z): &Cube) -> [Cube; 6] {
    [
        (x + 1, y, z),
        (x - 1, y, z),
        (x, y + 1, z),
        (x, y - 1, z),
        (x, y, z + 1),
        (x, y, z - 1),
    ]
}

struct OuterSurface {
    occupied: HashSet<Cube>,
    total: usize,
}

impl OuterSurface {
    fn new() -> Self {
        Self {
            occupied: HashSet::new(),
            total: 0,
        }
    }

    fn from_iter<'a>(iter: impl Iterator<Item = &'a Cube>) -> Self {
        let mut surface = Self::new();
        for cube in iter {
            surface.add_cube(cube);
        }
        surface
    }

    fn add_cube(&mut self, cube: &Cube) {
        if !self.occupied.contains(cube) {
            let blocked = neighbors(cube)
                .iter()
                .filter(|c| self.occupied.contains(c))
                .count();
            self.total = self.total + 6 - 2 * blocked;
            self.occupied.insert(*cube);
        }
    }
}

struct Limits {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    min_z: i32,
    max_z: i32,
}

impl Limits {
    fn new() -> Self {
        Self {
            min_x: i32::MAX,
            max_x: i32::MIN,
            min_y: i32::MAX,
            max_y: i32::MIN,
            min_z: i32::MAX,
            max_z: i32::MIN,
        }
    }

    fn adjust(&mut self, &(x, y, z): &Cube) {
        self.min_x = self.min_x.min(x);
        self.max_x = self.max_x.max(x);
        self.min_y = self.min_y.min(y);
        self.max_y = self.max_y.max(y);
        self.min_z = self.min_z.min(z);
        self.max_z = self.max_z.max(z);
    }

    fn x_range(&self) -> RangeInclusive<i32> {
        self.min_x..=self.max_x
    }

    fn y_range(&self) -> RangeInclusive<i32> {
        self.min_y..=self.max_y
    }

    fn z_range(&self) -> RangeInclusive<i32> {
        self.min_z..=self.max_z
    }

    fn is_within_limits(&self, (x, y, z): &Cube) -> bool {
        self.x_range().contains(x) && self.y_range().contains(y) && self.z_range().contains(z)
    }
}

#[aoc(day18, part1)]
pub fn part1(input: &str) -> usize {
    let cubes = parse_input(input);
    OuterSurface::from_iter(cubes.iter()).total
}

fn trapped_surface(from: &Cube, limits: &Limits, visited: &mut HashSet<Cube>) -> usize {
    if visited.contains(from) {
        return 0;
    }
    let mut touches_limit = false;
    let mut surface = OuterSurface::new();
    surface.add_cube(from);

    let mut q = VecDeque::new();
    q.push_back(*from);

    while let Some(cube) = q.pop_front() {
        for next in neighbors(&cube) {
            if visited.contains(&next) {
                continue;
            }
            if !limits.is_within_limits(&next) {
                touches_limit = true;
                continue;
            }
            surface.add_cube(&next);
            q.push_back(next);
            visited.insert(next);
        }
    }
    if touches_limit {
        0
    } else {
        surface.total
    }
}

#[aoc(day18, part2)]
pub fn part2(input: &str) -> usize {
    let cubes = parse_input(input);
    let mut limits = Limits::new();
    for cube in &cubes {
        limits.adjust(cube);
    }

    let mut visited: HashSet<Cube> = cubes.iter().copied().collect();
    let mut result = OuterSurface::from_iter(cubes.iter()).total;

    for x in limits.x_range() {
        for y in limits.y_range() {
            for z in limits.z_range() {
                result -= trapped_surface(&(x, y, z), &limits, &mut visited);
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let example = include_str!("examples/day18.txt");
        assert_eq!(part1(example), 64);
    }

    #[test]
    fn test_example_part2() {
        let example = include_str!("examples/day18.txt");
        assert_eq!(part2(example), 58);
    }
}
