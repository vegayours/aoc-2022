use aoc_runner_derive::aoc;

type Stacks = Vec<Vec<u8>>;

fn parse_stacks(lines: &[&str]) -> Stacks {
    let mut lines = lines.iter().rev();
    let first = lines.next().unwrap();
    let n = first.len() / 4 + 1;
    let mut stacks: Stacks = vec![Vec::new(); first.len() / 4 + 1];
    for line in lines {
        for i in 0..n {
            let pos = 1 + i * 4;
            if pos < line.len() && line.as_bytes()[pos] != b' ' {
                stacks[i].push(line.as_bytes()[pos]);
            }
        }
    }
    stacks
}

#[derive(Debug)]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

impl Move {
    fn parse(line: &str) -> Move {
        let parts: Vec<&str> = line.split_whitespace().collect();
        Move {
            amount: parts[1].parse::<usize>().unwrap(),
            from: parts[3].parse::<usize>().unwrap() - 1,
            to: parts[5].parse::<usize>().unwrap() - 1,
        }
    }
}

fn parse_input(input: &str) -> (Stacks, Vec<Move>) {
    let stack_lines: Vec<&str> = input.lines().take_while(|l| !l.trim().is_empty()).collect();
    let stacks = parse_stacks(&stack_lines);
    let moves = input
        .lines()
        .skip(stack_lines.len() + 1)
        .map(|l| Move::parse(l))
        .collect();
    (stacks, moves)
}

fn get_stack_refs(stacks: &mut Stacks, from: usize, to: usize) -> (&mut Vec<u8>, &mut Vec<u8>) {
    if from < to {
        let (left, right) = stacks.as_mut_slice().split_at_mut(to);
        (&mut left[from], &mut right[0])
    } else {
        let (left, right) = stacks.as_mut_slice().split_at_mut(from);
        (&mut right[0], &mut left[to])
    }
}

fn apply_move_part1(stacks: &mut Stacks, m: &Move) {
    if m.from == m.to {
        return;
    }
    let (from, to) = get_stack_refs(stacks, m.from, m.to);
    for _ in 0..m.amount {
        if let Some(x) = from.pop() {
            to.push(x);
        } else {
            break;
        }
    }
}

fn apply_move_part2(stacks: &mut Stacks, m: &Move) {
    if m.from == m.to {
        return;
    }
    let (from, to) = get_stack_refs(stacks, m.from, m.to);
    let start = from.len() - std::cmp::min(from.len(), m.amount);
    to.extend_from_slice(&from[start..]);
    from.truncate(start);
}

#[aoc(day05, part1)]
pub fn part1(input: &str) -> String {
    let (mut stacks, moves) = parse_input(input);
    for m in moves {
        apply_move_part1(&mut stacks, &m);
    }
    let bytes: Vec<u8> = stacks.iter().flat_map(|s| s.last().copied()).collect();
    String::from_utf8(bytes).unwrap()
}

#[aoc(day05, part2)]
pub fn part2(input: &str) -> String {
    let (mut stacks, moves) = parse_input(input);
    for m in moves {
        apply_move_part2(&mut stacks, &m);
    }
    let bytes: Vec<u8> = stacks.iter().flat_map(|s| s.last().copied()).collect();
    String::from_utf8(bytes).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let example = include_str!("examples/day05.txt");
        assert_eq!(part1(example), "CMZ");
    }

    #[test]
    fn test_example_part2() {
        let example = include_str!("examples/day05.txt");
        assert_eq!(part2(example), "MCD");
    }
}
