use aoc_runner_derive::aoc;

enum Instruction {
    Noop,
    Addx(i32),
}

fn parse(input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for line in input.lines().map(str::trim) {
        if line == "noop" {
            instructions.push(Instruction::Noop);
        } else if line.starts_with("addx") {
            if let Some(arg) = line.split_whitespace().nth(1) {
                instructions.push(Instruction::Noop);
                instructions.push(Instruction::Addx(arg.parse().unwrap()));
            }
        } else {
            panic!("Wrong input line: {}", line);
        }
    }
    instructions
}

#[aoc(day10, part1)]
pub fn part1(input: &str) -> i32 {
    let instructions = parse(input);
    let mut target_cycle = 20;
    let mut result: i32 = 0;
    let mut register: i32 = 1;
    for (cycle, instruction) in instructions.iter().enumerate().map(|(i, x)| (i + 1, x)) {
        if cycle == target_cycle {
            result += (cycle as i32) * register;
            target_cycle += 40;
        }
        if let Instruction::Addx(x) = instruction {
            register += x;
        }
    }
    result
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> String {
    let instructions = parse(input);
    let mut crt = vec![vec![b'.'; 40]; 6];
    let mut sprite_pos: i32 = 0;
    for (cycle, instruction) in instructions.iter().enumerate() {
        if (sprite_pos..=sprite_pos + 2).contains(&(cycle as i32 % 40)) {
            crt[cycle / 40][cycle % 40] = b'#';
        }
        if let Instruction::Addx(x) = instruction {
            sprite_pos += x;
        }
    }
    std::iter::once(&Vec::<u8>::new())
        .chain(crt.iter())
        .flat_map(|x| std::str::from_utf8(x))
        .collect::<Vec<&str>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let example = include_str!("examples/day10.txt");
        assert_eq!(part1(example), 13140);
    }

    #[test]
    fn test_example_part2() {
        let example = include_str!("examples/day10.txt");
        let result = r#"
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."#;
        assert_eq!(part2(example), String::from(result));
    }
}
