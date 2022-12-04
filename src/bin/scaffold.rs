/*
 * This file contains template code.
 * There is no need to edit this file unless you want to change template functionality.
 */
use std::{
    fs::{File, OpenOptions},
    io::Write,
    process,
};

const MODULE_TEMPLATE: &str = r###"use aoc_runner_derive::aoc;

#[aoc(day{DAY}, part1, imperative)]
pub fn part1(input: &str) -> i32 {
    42
}

#[aoc(day{DAY}, part2)]
pub fn part2(input: &str) -> i32 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let example = include_str!("examples/day{DAY}.txt");
        assert_eq!(part1(example), todo!());
    }

    #[test]
    fn test_example_part2() {
        let example = include_str!("examples/day{DAY}.txt");
        assert_eq!(part2(example), todo!());
    }
}
"###;

fn parse_args() -> Result<u8, pico_args::Error> {
    let mut args = pico_args::Arguments::from_env();
    args.free_from_str()
}

fn safe_create_file(path: &str) -> Result<File, std::io::Error> {
    OpenOptions::new().write(true).create_new(true).open(path)
}

fn create_file(path: &str) -> Result<File, std::io::Error> {
    OpenOptions::new().write(true).create(true).open(path)
}

fn main() {
    let day = match parse_args() {
        Ok(day) => day,
        Err(_) => {
            eprintln!("Need to specify a day (as integer). example: `cargo scaffold 7`");
            process::exit(1);
        }
    };

    let day_padded = format!("{:02}", day);

    let example_path = format!("src/examples/day{}.txt", day_padded);
    let module_path = format!("src/day{}.rs", day_padded);

    let mut file = match safe_create_file(&module_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create module file: {}", e);
            process::exit(1);
        }
    };

    match file.write_all(MODULE_TEMPLATE.replace("{DAY}", &day_padded).as_bytes()) {
        Ok(_) => {
            println!("Created module file \"{}\"", &module_path);
        }
        Err(e) => {
            eprintln!("Failed to write module contents: {}", e);
            process::exit(1);
        }
    }

    match create_file(&example_path) {
        Ok(_) => {
            println!("Created empty example file \"{}\"", &example_path);
        }
        Err(e) => {
            eprintln!("Failed to create example file: {}", e);
            process::exit(1);
        }
    }

    println!("---");
    println!("Type `cargo aoc -d {}` to run your solution.", &day_padded);
}
