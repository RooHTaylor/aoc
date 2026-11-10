use aoc_common::Options;

mod day01;

pub type Solver = fn(&str, &Options) -> Result<String, String>;

pub fn solver(day: u8, part: u8) -> Option<Solver> {
    match (day, part) {
        (1, 1) => Some(day01::part1),
        (1, 2) => Some(day01::part2),
        _ => None,
    }
}