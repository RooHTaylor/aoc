use aoc_common::Options;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

pub type Solver = fn(&str, &Options) -> Result<String, String>;

pub fn solver(day: u8, part: u8) -> Option<Solver> {
    match (day, part) {
        (1, 1) => Some(day01::part1),
        (1, 2) => Some(day01::part2),
        (2, 1) => Some(day02::part1),
        (2, 2) => Some(day02::part2),
        (3, 1) => Some(day03::part1),
        (3, 2) => Some(day03::part2),
        (4, 1) => Some(day04::part1),
        (4, 2) => Some(day04::part2),
        (5, 1) => Some(day05::part1),
        (5, 2) => Some(day05::part2),
        (6, 1) => Some(day06::part1),
        (6, 2) => Some(day06::part2),
        (7, 1) => Some(day07::part1),
        (7, 2) => Some(day07::part2),
        _ => None,
    }
}