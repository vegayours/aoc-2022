use std::collections::HashSet;

use aoc_runner_derive::aoc;
use itertools::Itertools;

type T = i32;
type Point = (T, T);

const Y_LIM: T = 7;
const DEBUG: bool = false;

#[derive(Debug)]
struct Figure {
    points: Vec<Point>,
}

fn parse_figure(f: &[&str]) -> Figure {
    let mut points = Vec::new();
    for (x, line) in f.iter().rev().enumerate() {
        for (y, &b) in line.as_bytes().iter().enumerate() {
            if b == b'#' {
                points.push((x as T, y as T))
            }
        }
    }
    Figure { points }
}

fn get_figures() -> Vec<Figure> {
    vec![
        parse_figure(&["####"]),
        parse_figure(&[".#.", "###", ".#."]),
        parse_figure(&["..#", "..#", "###"]),
        parse_figure(&["#", "#", "#", "#"]),
        parse_figure(&["##", "##"]),
    ]
}

struct State<'a> {
    occupied: HashSet<Point>,
    shift: Box<dyn Iterator<Item = u8> + 'a>,
    max_height: T,
}

impl<'a> State<'a> {
    fn new(input: &'a str) -> Self {
        State {
            occupied: (0..7).map(|y| (0, y)).collect(),
            shift: Box::new(input.trim_end().bytes().cycle()),
            max_height: 0,
        }
    }

    fn is_valid(&self, (dx, dy): Point, figure: &Figure) -> bool {
        figure.points.iter().all(|&(x, y)| {
            let (x, y) = (x + dx, y + dy);
            (0..Y_LIM).contains(&y) && y < Y_LIM && !self.occupied.contains(&(x, y))
        })
    }

    fn apply_shift(&mut self, dx: T, dy: &mut T, figure: &Figure) {
        match self.shift.next() {
            Some(b'<') => {
                if self.is_valid((dx, *dy - 1), figure) {
                    *dy -= 1;
                }
            }
            Some(b'>') => {
                if self.is_valid((dx, *dy + 1), figure) {
                    *dy += 1;
                }
            }
            x => panic!("Unexpected shift: {x:?}"),
        }
    }

    fn lock(&mut self, figure: &Figure) {
        let (mut dx, mut dy) = (self.max_height + 4, 2);
        loop {
            if DEBUG {
                self.show_with_figure(
                    figure
                        .points
                        .iter()
                        .map(|&(x, y)| (x + dx, y + dy))
                        .collect(),
                );
            }
            self.apply_shift(dx, &mut dy, figure);
            let is_blocked = figure
                .points
                .iter()
                .any(|&(x, y)| self.occupied.contains(&(x + dx - 1, y + dy)));
            if is_blocked {
                for &(x, y) in &figure.points {
                    let (x, y) = (x + dx, y + dy);
                    self.occupied.insert((x, y));
                    self.max_height = self.max_height.max(x);
                }
                break;
            } else {
                dx -= 1;
            }
        }
    }

    fn show(&self) {
        self.show_with_figure(HashSet::new());
    }

    fn show_with_figure(&self, f: HashSet<Point>) {
        let lim = f.iter().map(|&(x, _y)| x).max().unwrap_or(self.max_height);

        for x in (0..=lim).rev() {
            let mut row = vec![b'.'; Y_LIM as usize];
            for y in 0..Y_LIM {
                if self.occupied.contains(&(x, y)) {
                    row[y as usize] = b'#';
                } else if f.contains(&(x, y)) {
                    row[y as usize] = b'@';
                }
            }
            println!("{}", std::str::from_utf8(&row).unwrap());
        }
    }
}

#[aoc(day17, part1)]
pub fn part1(input: &str) -> i32 {
    let figures = get_figures();
    let mut state = State::new(input);

    for figure in figures.iter().cycle().take(2022) {
        state.lock(figure);
        if DEBUG {
            state.show();
        }
    }

    state.max_height
}

fn detect_cycle(height: &[i32], limit: usize, cycle_threshod: usize) -> Option<(usize, usize)> {
    for cycle_width in 4..=(limit / cycle_threshod) {
        let mut iter = height
            .iter()
            .rev()
            .step_by(cycle_width)
            .tuple_windows()
            .map(|(l, c)| l - c);
        let diff = iter.next().unwrap();
        if iter.take_while(|&d| d == diff).take(cycle_threshod).count() == cycle_threshod {
            return Some((cycle_width, diff as usize));
        }
    }
    None
}

#[aoc(day17, part2)]
pub fn part2(input: &str) -> usize {
    let figures = get_figures();
    let mut state = State::new(input);

    const LIMIT: usize = 100_000;
    const CYCLE_THRESHOLD: usize = 10;

    let mut height: Vec<T> = Vec::with_capacity(LIMIT);

    for figure in figures.iter().cycle().take(LIMIT) {
        state.lock(figure);
        height.push(state.max_height);
    }

    if let Some((cycle_width, height_diff)) = detect_cycle(&height, LIMIT, CYCLE_THRESHOLD) {
        const STEPS: usize = 1_000_000_000_000;
        let rest = STEPS - LIMIT;
        let skip = cycle_width - rest % cycle_width;
        let result = *height.iter().rev().nth(skip).unwrap() as usize
            + (1 + rest / cycle_width) * height_diff;
        return result;
    }
    panic!("No solution");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let example = include_str!("examples/day17.txt");
        assert_eq!(part1(example), 3068);
    }

    #[test]
    fn test_example_part2() {
        let example = include_str!("examples/day17.txt");
        assert_eq!(part2(example), 1514285714288);
    }
}
