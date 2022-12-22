use aoc_runner_derive::aoc;
use itertools::Itertools;

struct Grid<'a> {
    data: Vec<&'a [u8]>,
    row_start: Vec<usize>,
    row_end: Vec<usize>,
    col_start: Vec<usize>,
    col_end: Vec<usize>,
}

impl<'a> Grid<'a> {
    fn new(input: &'a str) -> Grid<'a> {
        let data: Vec<&[u8]> = input.lines().map(|s| s.as_bytes()).collect();
        let rows = data.len();
        let cols = data.iter().map(|r| r.len()).max().unwrap();
        let mut row_start = vec![0; rows];
        let mut row_end = vec![cols - 1; rows];
        let mut col_start = vec![0; cols];
        let mut col_end = vec![rows - 1; cols];

        for i in 0..rows {
            let row = data[i];
            for j in 0..row.len() {
                if row[j] == b'.' || row[j] == b'#' {
                    if j + 1 >= row.len() || row[j + 1] == b' ' {
                        row_end[i] = j;
                    }
                    if j > 0 && row[j - 1] == b' ' {
                        row_start[i] = j;
                    }
                    if i + 1 < rows && (j >= data[i + 1].len() || data[i + 1][j] == b' ') {
                        col_end[j] = i;
                    }
                    if i > 0 && (j >= data[i - 1].len() || data[i - 1][j] == b' ')  {
                        col_start[j] = i;
                    }
                }
            }
        }

        Grid {
            data,
            row_start,
            row_end,
            col_start,
            col_end,
        }
    }

    fn next_step(&self, row: usize, col: usize, orientation: &Orientation) -> (usize, usize) {
        match orientation {
            Orientation::Up => {
                if row == self.col_start[col] {
                    (self.col_end[col], col)
                } else {
                    (row - 1, col)
                }
            }
            Orientation::Right => {
                if col == self.row_end[row] {
                    (row, self.row_start[row])
                } else {
                    (row, col + 1)
                }
            }
            Orientation::Down => {
                if row == self.col_end[col] {
                    (self.col_start[col], col)
                } else {
                    (row + 1, col)
                }
            }
            Orientation::Left => {
                if col == self.row_start[row] {
                    (row, self.row_end[row])
                } else {
                    (row, col - 1)
                }
            }
        }
    }
}

#[derive(Debug)]
enum Move {
    Advance(usize),
    RotateLeft,
    RotateRight,
}

#[derive(Debug)]
enum Orientation {
    Up,
    Right,
    Down,
    Left,
}

impl Orientation {
    fn rotate_right(&self) -> Orientation {
        match self {
            Orientation::Up => Orientation::Right,
            Orientation::Right => Orientation::Down,
            Orientation::Down => Orientation::Left,
            Orientation::Left => Orientation::Up,
        }
    }

    fn rotate_left(&self) -> Orientation {
        match self {
            Orientation::Up => Orientation::Left,
            Orientation::Right => Orientation::Up,
            Orientation::Down => Orientation::Right,
            Orientation::Left => Orientation::Down,
        }
    }

    fn score(&self) -> usize {
        match self {
            Orientation::Up => 3,
            Orientation::Right => 0,
            Orientation::Down => 1,
            Orientation::Left => 2,
        }
    }
}

#[derive(Debug)]
struct State {
    orientation: Orientation,
    row: usize,
    col: usize,
}

impl State {
    fn new(grid: &Grid) -> State {
        let col = grid.data[0]
            .iter()
            .enumerate()
            .find(|&(_j, val)| *val == b'.')
            .unwrap()
            .0;
        State {
            orientation: Orientation::Right,
            row: 0,
            col,
        }
    }

    fn apply_move(&mut self, m: &Move, grid: &Grid) {
        match m {
            Move::Advance(steps) => {
                for _ in 0..*steps {
                    let (next_row, next_col) =
                        grid.next_step(self.row, self.col, &self.orientation);
                    match grid.data[next_row][next_col] {
                        b'.' => {
                            self.row = next_row;
                            self.col = next_col;
                        }
                        b'#' => {
                            break;
                        }
                        c => {
                            panic!(
                                "Unexpected move '{}', from: {:?}, to: {:?}",
                                c as char,
                                (self.row, self.col),
                                (next_row, next_col)
                            );
                        }
                    }
                }
            }
            Move::RotateLeft => self.orientation = self.orientation.rotate_left(),
            Move::RotateRight => self.orientation = self.orientation.rotate_right(),
        }
    }
}

fn parse_moves(input: &str) -> Vec<Move> {
    let nums = input
        .split(|c: char| c == 'L' || c == 'R')
        .flat_map(|n| n.parse::<usize>())
        .map(|step| Move::Advance(step));
    let rot = input.chars().flat_map(|c| match c {
        'L' => Some(Move::RotateLeft),
        'R' => Some(Move::RotateRight),
        _ => None,
    });
    let res = nums.interleave(rot).collect();
    res
}

fn parse_input<'a>(input: &'a str) -> (Grid<'a>, Vec<Move>) {
    let mut parts = input.split("\n\n");
    let grid = Grid::new(parts.next().unwrap());
    let moves = parse_moves(parts.next().unwrap());
    (grid, moves)
}

#[aoc(day22, part1)]
pub fn part1(input: &str) -> usize {
    let (grid, moves) = parse_input(input);
    let mut state = State::new(&grid);
    for m in moves {
        state.apply_move(&m, &grid);
    }

    1000 * (state.row + 1) + 4 * (state.col + 1) + state.orientation.score()
}

#[aoc(day22, part2)]
pub fn part2(input: &str) -> i32 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let example = include_str!("examples/day22.txt");
        assert_eq!(part1(example), 6032);
    }

    #[test]
    fn test_example_part2() {
        let example = include_str!("examples/day22.txt");
        assert_eq!(part2(example), 42);
    }
}
