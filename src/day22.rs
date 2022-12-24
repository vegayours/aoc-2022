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
                    if i > 0 && (j >= data[i - 1].len() || data[i - 1][j] == b' ') {
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
}

#[derive(Debug)]
enum Move {
    Advance(usize),
    RotateLeft,
    RotateRight,
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
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
}

type WrapFn = fn(State, &Grid) -> State;
type TranslateFn = fn((usize, usize), Orientation) -> ((usize, usize), Orientation, bool);

#[cfg(test)]
fn translate_part2_test(
    (from_row_part, from_col_part): (usize, usize),
    orientation: Orientation,
) -> ((usize, usize), Orientation, bool) {
    match ((from_row_part, from_col_part), orientation) {
        // Up
        ((1, 0), Orientation::Up) => ((0, 2), Orientation::Down, true),
        ((1, 1), Orientation::Up) => ((0, 2), Orientation::Right, false),
        ((0, 2), Orientation::Up) => ((1, 0), Orientation::Down, true),
        ((2, 3), Orientation::Up) => ((0, 2), Orientation::Left, true),
        // Right
        ((0, 2), Orientation::Right) => ((2, 3), Orientation::Left, true),
        ((1, 2), Orientation::Right) => ((2, 3), Orientation::Down, true),
        ((2, 3), Orientation::Right) => ((0, 2), Orientation::Left, true),
        // Down
        ((1, 0), Orientation::Down) => ((2, 2), Orientation::Up, true),
        ((1, 1), Orientation::Down) => ((2, 2), Orientation::Right, true),
        ((2, 2), Orientation::Down) => ((1, 0), Orientation::Up, true),
        ((2, 3), Orientation::Down) => ((1, 0), Orientation::Right, true),
        // Left
        ((0, 2), Orientation::Left) => ((1, 1), Orientation::Down, false),
        ((1, 0), Orientation::Left) => ((2, 3), Orientation::Up, true),
        ((2, 2), Orientation::Left) => ((1, 1), Orientation::Up, true),
        // Unsupported
        from => panic!("Unsupported move: {from:?}"),
    }
}

fn translate_part2(
    (from_row_part, from_col_part): (usize, usize),
    orientation: Orientation,
) -> ((usize, usize), Orientation, bool) {
    match ((from_row_part, from_col_part), orientation) {
        ((0, 2), Orientation::Up) => ((3, 0), Orientation::Up, false),
        ((0, 2), Orientation::Right) => ((2, 1), Orientation::Left, true),
        ((0, 2), Orientation::Down) => ((1, 1), Orientation::Left, false),

        ((0, 1), Orientation::Up) => ((3, 0), Orientation::Right, false),
        ((0, 1), Orientation::Left) => ((2, 0), Orientation::Right, true),

        ((1, 1), Orientation::Left) => ((2, 0), Orientation::Down, false),
        ((1, 1), Orientation::Right) => ((0, 2), Orientation::Up, false),

        ((2, 1), Orientation::Right) => ((0, 2), Orientation::Left, true),
        ((2, 1), Orientation::Down) => ((3, 0), Orientation::Left, false),

        ((2, 0), Orientation::Up) => ((1, 1), Orientation::Right, false),
        ((2, 0), Orientation::Left) => ((0, 1), Orientation::Right, true),

        ((3, 0), Orientation::Left) => ((0, 1), Orientation::Down, false),
        ((3, 0), Orientation::Down) => ((0, 2), Orientation::Down, false),
        ((3, 0), Orientation::Right) => ((2, 1), Orientation::Up, false),

        // Unsupported
        from => panic!("Unsupported move: {from:?}"),
    }
}

#[cfg(test)]
fn wrap_p2_test(state: State, grid: &Grid) -> State {
    wrap_p2_impl(grid.data.len() / 3, state, grid, translate_part2_test)
}

fn wrap_p2(state: State, grid: &Grid) -> State {
    wrap_p2_impl(grid.data.len() / 4, state, grid, translate_part2)
}

fn wrap_p2_impl(cube_width: usize, state: State, grid: &Grid, translate_fn: TranslateFn) -> State {
    let rel_row = state.row % cube_width;
    let rel_col = state.col % cube_width;
    let from_row_part = state.row / cube_width;
    let from_col_part = state.col / cube_width;

    let ((to_row_part, to_col_part), orientation, inverse) =
        translate_fn((from_row_part, from_col_part), state.orientation);
    let rel = match state.orientation {
        Orientation::Up | Orientation::Down => rel_col,
        Orientation::Left | Orientation::Right => rel_row,
    };
    let to_part = match orientation {
        Orientation::Up | Orientation::Down => to_col_part,
        Orientation::Left | Orientation::Right => to_row_part,
    };
    let to = if inverse {
        (to_part + 1) * cube_width - rel - 1
    } else {
        to_part * cube_width + rel
    };
    let (row, col) = match orientation {
        Orientation::Up => (grid.col_end[to], to),
        Orientation::Down => (grid.col_start[to], to),
        Orientation::Right => (to, grid.row_start[to]),
        Orientation::Left => (to, grid.row_end[to]),
    };
    State {
        row,
        col,
        orientation,
    }
}

fn wrap_p1(state: State, grid: &Grid) -> State {
    let State {
        row,
        col,
        orientation,
    } = state;
    match orientation {
        Orientation::Up => State {
            row: grid.col_end[col],
            ..state
        },
        Orientation::Right => State {
            col: grid.row_start[row],
            ..state
        },
        Orientation::Down => State {
            row: grid.col_start[col],
            ..state
        },
        Orientation::Left => State {
            col: grid.row_end[row],
            ..state
        },
    }
}

fn next_step(state: State, grid: &Grid, wrap_fn: WrapFn) -> State {
    let State {
        row,
        col,
        orientation,
    } = state;
    match orientation {
        Orientation::Up => {
            if row == grid.col_start[col] {
                wrap_fn(state, grid)
            } else {
                State {
                    row: row - 1,
                    ..state
                }
            }
        }
        Orientation::Right => {
            if col == grid.row_end[row] {
                wrap_fn(state, grid)
            } else {
                State {
                    col: col + 1,
                    ..state
                }
            }
        }
        Orientation::Down => {
            if row == grid.col_end[col] {
                wrap_fn(state, grid)
            } else {
                State {
                    row: row + 1,
                    ..state
                }
            }
        }
        Orientation::Left => {
            if col == grid.row_start[row] {
                wrap_fn(state, grid)
            } else {
                State {
                    col: col - 1,
                    ..state
                }
            }
        }
    }
}

fn apply_move(mut state: State, m: &Move, grid: &Grid, wrap_fn: WrapFn) -> State {
    match m {
        Move::Advance(steps) => {
            for _ in 0..*steps {
                let next_state = next_step(state, grid, wrap_fn);
                match grid.data[next_state.row][next_state.col] {
                    b'.' => {
                        state = next_state;
                    }
                    b'#' => {
                        break;
                    }
                    c => {
                        panic!(
                            "Unexpected move '{}', from: {:?}, to: {:?}",
                            c as char, state, next_state
                        );
                    }
                }
            }
            state
        }
        Move::RotateLeft => State {
            orientation: state.orientation.rotate_left(),
            ..state
        },
        Move::RotateRight => State {
            orientation: state.orientation.rotate_right(),
            ..state
        },
    }
}

fn parse_moves(input: &str) -> Vec<Move> {
    let input = input.trim_end();
    let nums = input
        .split(|c: char| c == 'L' || c == 'R')
        .flat_map(|n| n.parse::<usize>())
        .map(Move::Advance);
    let rot = input.chars().flat_map(|c| match c {
        'L' => Some(Move::RotateLeft),
        'R' => Some(Move::RotateRight),
        _ => None,
    });
    nums.interleave(rot).collect()
}

fn parse_input(input: &'_ str) -> (Grid<'_>, Vec<Move>) {
    let mut parts = input.split("\n\n");
    let grid = Grid::new(parts.next().unwrap());
    let moves = parse_moves(parts.next().unwrap());
    (grid, moves)
}

fn solve(input: &str, wrap_fn: WrapFn) -> usize {
    let (grid, moves) = parse_input(input);
    let mut state = State::new(&grid);
    for m in moves {
        state = apply_move(state, &m, &grid, wrap_fn);
    }

    1000 * (state.row + 1) + 4 * (state.col + 1) + state.orientation.score()
}

#[aoc(day22, part1)]
pub fn part1(input: &str) -> usize {
    solve(input, wrap_p1)
}

#[aoc(day22, part2)]
pub fn part2(input: &str) -> usize {
    solve(input, wrap_p2)
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
        assert_eq!(solve(example, wrap_p2_test), 5031);
    }
}
