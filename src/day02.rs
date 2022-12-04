use aoc_runner_derive::aoc;

#[derive(PartialEq, Clone, Debug)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Copy for Choice {}

impl Choice {
    fn parse(s: &str) -> Choice {
        match s {
            "A" | "X" => Choice::Rock,
            "B" | "Y" => Choice::Paper,
            "C" | "Z" => Choice::Scissors,
            _ => panic!("Unsupported choice string: {s}"),
        }
    }
    fn to_outcome(&self) -> Outcome {
        match self {
            Choice::Rock => Outcome::Lose,
            Choice::Paper => Outcome::Draw,
            Choice::Scissors => Outcome::Win,
        }
    }
    fn matchin_choice(&self, outcome: &Outcome) -> Choice {
        match outcome {
            Outcome::Draw => *self,
            Outcome::Win => match self {
                Choice::Rock => Choice::Paper,
                Choice::Paper => Choice::Scissors,
                Choice::Scissors => Choice::Rock,
            },
            Outcome::Lose => match self {
                Choice::Rock => Choice::Scissors,
                Choice::Paper => Choice::Rock,
                Choice::Scissors => Choice::Paper,
            },
        }
    }
    fn score(&self) -> i32 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }
}

#[derive(Debug)]
struct Round {
    opponent: Choice,
    player: Choice,
}

#[derive(Debug)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Outcome {
    fn score(&self) -> i32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }
}

impl Round {
    fn parse(row: &str) -> Round {
        match row.split(" ").collect::<Vec<&str>>()[..] {
            [opponent, player] => Round {
                opponent: Choice::parse(opponent),
                player: Choice::parse(player),
            },
            _ => panic!("Unsupported row format: {row}"),
        }
    }
    fn outcome(&self) -> Outcome {
        match (&self.player, &self.opponent) {
            (x, y) if x == y => Outcome::Draw,
            (Choice::Rock, Choice::Scissors)
            | (Choice::Paper, Choice::Rock)
            | (Choice::Scissors, Choice::Paper) => Outcome::Win,
            _ => Outcome::Lose,
        }
    }
    fn part1_score(&self) -> i32 {
        self.player.score() + self.outcome().score()
    }
    fn part2_score(&self) -> i32 {
        let outcome = self.player.to_outcome();
        outcome.score() + self.opponent.matchin_choice(&outcome).score()
    }
}

fn parse_rounds(input: &str) -> Vec<Round> {
    input.lines().map(|x| Round::parse(x.trim())).collect()
}

#[aoc(day02, part1)]
pub fn part1(input: &str) -> i32 {
    parse_rounds(input).iter().map(Round::part1_score).sum()
}

#[aoc(day02, part2)]
pub fn part2(input: &str) -> i32 {
    parse_rounds(input).iter().map(Round::part2_score).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let example = include_str!("examples/day02.txt");
        assert_eq!(part1(example), 15);
    }

    #[test]
    fn test_example_part2() {
        let example = include_str!("examples/day02.txt");
        assert_eq!(part2(example), 12);
    }
}
