use aoc_runner_derive::aoc;
use rayon::prelude::*;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"Blueprint \d+: Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
}

#[derive(Debug, Clone, Copy, Default)]
struct Blueprint {
    ore: u16,
    clay: u16,
    obsidian: (u16, u16),
    geode: (u16, u16),
}

fn parse_blueprint(line: &str) -> Blueprint {
    let captures: Vec<u16> = RE
        .captures(line)
        .unwrap()
        .iter()
        .skip(1)
        .flat_map(|c| c.unwrap().as_str().parse())
        .collect();
    Blueprint {
        ore: captures[0],
        clay: captures[1],
        obsidian: (captures[2], captures[3]),
        geode: (captures[4], captures[5]),
    }
}

#[derive(Default, Clone, Debug)]
struct State {
    ore_robots: u16,
    clay_robots: u16,
    obsidian_robots: u16,
    geode_robots: u16,

    ore: u16,
    clay: u16,
    obsidian: u16,
    geode: u16,
}

impl State {
    fn new() -> Self {
        Self {
            ore_robots: 1,
            ..Default::default()
        }
    }

    fn add_ore_robot(&self, blueprint: &Blueprint) -> Option<Self> {
        if self.ore >= blueprint.ore {
            let adjusted = self.tick();
            Some(Self {
                ore_robots: adjusted.ore_robots + 1,
                ore: adjusted.ore - blueprint.ore,
                ..adjusted
            })
        } else {
            None
        }
    }

    fn add_clay_robot(&self, blueprint: &Blueprint) -> Option<Self> {
        if self.ore >= blueprint.clay {
            let adjusted = self.tick();
            Some(Self {
                clay_robots: adjusted.clay_robots + 1,
                ore: adjusted.ore - blueprint.clay,
                ..adjusted
            })
        } else {
            None
        }
    }

    fn add_obsidian_robot(&self, blueprint: &Blueprint) -> Option<Self> {
        let (ore, clay) = blueprint.obsidian;
        if self.ore >= ore && self.clay >= clay {
            let adjusted = self.tick();
            Some(Self {
                obsidian_robots: adjusted.obsidian_robots + 1,
                ore: adjusted.ore - ore,
                clay: adjusted.clay - clay,
                ..adjusted
            })
        } else {
            None
        }
    }

    fn add_geode_robot(&self, blueprint: &Blueprint) -> Option<Self> {
        let (ore, obsidian) = blueprint.geode;
        if self.ore >= ore && self.obsidian >= obsidian {
            let adjusted = self.tick();
            Some(Self {
                geode_robots: adjusted.geode_robots + 1,
                ore: adjusted.ore - ore,
                obsidian: adjusted.obsidian - obsidian,
                ..adjusted
            })
        } else {
            None
        }
    }

    fn tick(&self) -> Self {
        Self {
            ore: self.ore + self.ore_robots,
            clay: self.clay + self.clay_robots,
            obsidian: self.obsidian + self.obsidian_robots,
            geode: self.geode + self.geode_robots,
            ..*self
        }
    }
}

fn backtrack(
    minutes_left: u16,
    state: State,
    blueprint: &Blueprint,
    total_result: &mut u16,
) -> u16 {
    if minutes_left == 0 {
        if *total_result < state.geode {
            *total_result = state.geode;
        }
        return state.geode;
    }

    let mut result = state.geode;

    // Empiric pruning that works fine for test & my input.
    // TODO: Implement proper pruning or come up with DP.
    if state.geode + (4 + state.geode_robots) * minutes_left < *total_result {
        return result;
    }

    if let Some(next_state) = state.add_geode_robot(blueprint) {
        let next_result = backtrack(minutes_left - 1, next_state, blueprint, total_result);
        if next_result > result {
            result = next_result;
        }
    }
    if let Some(next_state) = state.add_obsidian_robot(blueprint) {
        let next_result = backtrack(minutes_left - 1, next_state, blueprint, total_result);
        if next_result > result {
            result = next_result;
        }
    }
    if let Some(next_state) = state.add_clay_robot(blueprint) {
        let next_result = backtrack(minutes_left - 1, next_state, blueprint, total_result);
        if next_result > result {
            result = next_result;
        }
    }
    if let Some(next_state) = state.add_ore_robot(blueprint) {
        let next_result = backtrack(minutes_left - 1, next_state, blueprint, total_result);
        if next_result > result {
            result = next_result;
        }
    }

    result.max(backtrack(
        minutes_left - 1,
        state.tick(),
        blueprint,
        total_result,
    ))
}

#[aoc(day19, part1)]
pub fn part1(input: &str) -> usize {
    let blueprints: Vec<Blueprint> = input.lines().map(parse_blueprint).collect();
    blueprints
        .par_iter()
        .enumerate()
        .map(|(i, blueprint)| {
            let mut res = 0;
            println!(
                "Result: {}, blueprint: {blueprint:?}",
                backtrack(24, State::new(), blueprint, &mut res)
            );
            (res as usize) * (i + 1)
        })
        .sum()
}

#[aoc(day19, part2)]
pub fn part2(input: &str) -> usize {
    let blueprints: Vec<Blueprint> = input.lines().map(parse_blueprint).collect();
    blueprints
        .par_iter()
        .take(3)
        .map(|blueprint| {
            let mut res = 0;
            println!(
                "Result: {}, blueprint: {blueprint:?}",
                backtrack(32, State::new(), blueprint, &mut res)
            );
            res as usize
        })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let example = include_str!("examples/day19.txt");
        assert_eq!(part1(example), 33);
    }

    #[test]
    fn test_example_part2() {
        let example = include_str!("examples/day19.txt");
        assert_eq!(part2(example), 56 * 62);
    }
}
