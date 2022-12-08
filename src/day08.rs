use aoc_runner_derive::aoc;

use std::cmp::max;

#[derive(Default, Clone)]
struct Cell {
    left: u8,
    right: u8,
    up: u8,
    down: u8,
}

impl Cell {
    fn min(&self) -> u8 {
        [self.left, self.right, self.up, self.down]
            .iter()
            .min()
            .copied()
            .unwrap()
    }
}

#[aoc(day08, part1)]
pub fn part1(input: &str) -> usize {
    let lines: Vec<&[u8]> = input.lines().map(str::trim).map(str::as_bytes).collect();
    let m = lines.len();
    let n = lines[0].len();
    let mut cells = vec![vec![Cell::default(); n]; m];
    for i in 1..m - 1 {
        for j in 1..n - 1 {
            cells[i][j].up = max(cells[i - 1][j].up, lines[i - 1][j]);
            cells[i][j].left = max(cells[i][j - 1].left, lines[i][j - 1]);
        }
    }
    for i in (1..m - 1).rev() {
        for j in (1..n - 1).rev() {
            cells[i][j].down = max(cells[i + 1][j].down, lines[i + 1][j]);
            cells[i][j].right = max(cells[i][j + 1].right, lines[i][j + 1]);
        }
    }
    let mut result = 2 * m + 2 * n - 4;
    for i in 1..m - 1 {
        for j in 1..n - 1 {
            if lines[i][j] > cells[i][j].min() {
                result += 1;
            }
        }
    }
    result
}

fn handle_iter<'a>(input: impl Iterator<Item = &'a u8>, output: impl Iterator<Item = &'a mut u32>) {
    let mut stack: Vec<(u8, usize)> = Vec::new();

    let res = input.enumerate().map(|(i, value)| {
        while !stack.is_empty() && stack.last().unwrap().0 < *value {
            stack.pop();
        }
        let res = if let Some((_, pos)) = stack.last() {
            i - pos
        } else {
            i
        };
        stack.push((*value, i));
        res as u32
    });
    for (out, mul) in output.zip(res) {
        *out *= mul;
    }
}

#[aoc(day08, part2)]
pub fn part2(input: &str) -> u32 {
    let lines: Vec<&[u8]> = input.lines().map(str::trim).map(str::as_bytes).collect();
    let m = lines.len();
    let n = lines[0].len();
    let mut cells = vec![vec![1u32; n]; m];
    for i in 1..m - 1 {
        handle_iter(lines[i].iter(), cells[i].iter_mut());
        handle_iter(lines[i].iter().rev(), cells[i].iter_mut().rev());
    }
    for j in 1..n - 1 {
        handle_iter(
            lines.iter().map(|l| &l[j]),
            cells.iter_mut().map(|l| &mut l[j]),
        );
        handle_iter(
            lines.iter().map(|l| &l[j]).rev(),
            cells.iter_mut().map(|l| &mut l[j]).rev(),
        );
    }
    cells
        .iter()
        .flat_map(|row| row.iter().max())
        .max()
        .copied()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let example = include_str!("examples/day08.txt");
        assert_eq!(part1(example), 21);
    }

    #[test]
    fn test_example_part2() {
        let example = include_str!("examples/day08.txt");
        assert_eq!(part2(example), 8);
    }
}
