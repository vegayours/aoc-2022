use aoc_runner_derive::aoc;

struct Snafu {
    // Digits in reverse oder.
    digits: Vec<i8>,
}

impl Snafu {
    fn zero() -> Snafu {
        Snafu { digits: vec![0] }
    }
    fn parse(input: &str) -> Snafu {
        let digits = input
            .trim_end()
            .as_bytes()
            .iter()
            .rev()
            .map(|b| match b {
                b'=' => -2,
                b'-' => -1,
                b'0' => 0,
                b'1' => 1,
                b'2' => 2,
                c => panic!("Unsupported src digit: {}", *c as char),
            })
            .collect();
        Snafu { digits }
    }

    fn as_string(&self) -> String {
        self.digits
            .iter()
            .rev()
            .map(|d| match d {
                -2 => '=',
                -1 => '-',
                0 => '0',
                1 => '1',
                2 => '2',
                d => panic!("Unsupported repr digit: {d}"),
            })
            .collect()
    }
}

fn wrap(val: i8) -> (i8, i8) {
    if val > 2 {
        (val - 5, 1)
    } else if val < -2 {
        (5 + val, -1)
    } else {
        (val, 0)
    }
}

fn add(left: &Snafu, right: &Snafu) -> Snafu {
    let mut res = Vec::new();
    let mut carry = 0;
    let mut left = left.digits.iter();
    let mut right = right.digits.iter();
    let mut val;
    loop {
        match (left.next(), right.next()) {
            (Some(l), Some(r)) => {
                (val, carry) = wrap(l + r + carry);
                res.push(val);
            }
            (Some(l), None) => {
                (val, carry) = wrap(l + carry);
                res.push(val);
            }
            (None, Some(r)) => {
                (val, carry) = wrap(r + carry);
                res.push(val);
            }
            (None, None) => {
                break;
            }
        }
    }
    if carry != 0 {
        res.push(carry);
    }
    while let Some(&0) = res.last() {
        res.pop();
    }
    Snafu { digits: res }
}

#[aoc(day25, part1)]
pub fn part1(input: &str) -> String {
    let mut res = Snafu::zero();
    for num in input.lines().map(Snafu::parse) {
        res = add(&res, &num);
    }
    res.as_string()
}

#[aoc(day25, part2)]
pub fn part2(_input: &str) -> i32 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let example = include_str!("examples/day25.txt");
        assert_eq!(part1(example), "2=-1=0");
    }

    #[test]
    fn test_example_part2() {
        let example = include_str!("examples/day25.txt");
        assert_eq!(part2(example), 42);
    }
}
