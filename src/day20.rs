use aoc_runner_derive::aoc;

fn read_input(input: &str) -> Vec<i64> {
    input.lines().flat_map(|l| l.parse()).collect()
}

fn mix(input: &Vec<i64>, times: usize) -> Vec<i64> {
    let mut pos: Vec<(usize, bool)> = (0..input.len()).map(|i| (i, false)).collect();
    let n = input.len() as i64;
    for _ in 0..times {
        while let Some((i, &(idx, _))) = pos
            .iter()
            .enumerate()
            .filter(|(_pos, (_, is_visited))| !is_visited)
            .min_by_key(|(_, (idx, _))| idx)
        {
            let i = i as i64;
            pos[i as usize] = (idx, true);
            let mut shift = input[idx] % (n - 1);
            if i + shift >= n {
                shift = shift - n + 1
            } else if i + shift < 0 {
                shift = n + shift - 1
            }
            let sign = shift.signum();
            for s in 0..shift.abs() {
                pos.swap((i + s * sign) as usize, (i + (s + 1) * sign) as usize);
            }
        }
        for (_idx, is_visited) in pos.iter_mut() {
            *is_visited = false;
        }
    }
    pos.iter().map(|(i, _)| input[*i]).collect()
}

#[aoc(day20, part1)]
pub fn part1(input: &str) -> i64 {
    let input = read_input(input);
    let res = mix(&input, 1);
    res.into_iter()
        .cycle()
        .skip_while(|&x| x != 0)
        .step_by(1000)
        .skip(1)
        .take(3)
        .sum()
}

#[aoc(day20, part2)]
pub fn part2(input: &str) -> i64 {
    let input: Vec<i64> = read_input(input).iter().map(|x| x * 811589153).collect();
    let res = mix(&input, 10);
    res.into_iter()
        .cycle()
        .skip_while(|&x| x != 0)
        .step_by(1000)
        .skip(1)
        .take(3)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let example = include_str!("examples/day20.txt");
        assert_eq!(part1(example), 3);
    }

    #[test]
    fn test_example_part2() {
        let example = include_str!("examples/day20.txt");
        assert_eq!(part2(example), 1623178306);
    }
}
