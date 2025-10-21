use shared::*;
use clap::Parser;
use std::collections::HashSet;

/// Advent Of Code 2016 Day 01 Part 1 and 2
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input file containing instructions
    #[arg(short, long)]
    filename: String,

    /// Toggle debug messages
    #[arg(long, default_value_t = false)]
    debug: bool,
}

fn main() {
    // Parse args
    let args = Args::parse();

    // Load input file and parse.
    let file_contents = load_input_file(&args.filename);
    let instructions = parse_instructions(args.debug, &file_contents);

    if args.debug { println!("Instructions: {:?}", instructions);}

    find_hq(args.debug, &instructions);
}

fn find_hq(
    debug: bool,
    instructions: &Vec<Instruction>
) {
    let mut direction: Direction = Direction::NORTH;
    let mut x: isize = 0;
    let mut y: isize = 0;
    if debug { println!("Starting at {},{} facing {:?}", x, y, direction); }
    let mut history: HashSet<(isize, isize)> = HashSet::new();
    history.insert((x,y));
    let mut first_double_visit: Option<(isize, isize)> = None;

    for instruction in instructions {

        // Find our new direction.
        match instruction.turn {
            Turn::LEFT => {
                if debug { println!("Turning left"); }
                match direction {
                    Direction::NORTH => {
                        direction = Direction::WEST;
                    },
                    Direction::EAST => {
                        direction = Direction::NORTH;
                    },
                    Direction::SOUTH => {
                        direction = Direction::EAST;
                    },
                    Direction::WEST => {
                        direction = Direction::SOUTH;
                    },
                }
            },
            Turn::RIGHT => {
                if debug { println!("Turning right"); }
                match direction {
                    Direction::NORTH => {
                        direction = Direction::EAST;
                    },
                    Direction::EAST => {
                        direction = Direction::SOUTH;
                    },
                    Direction::SOUTH => {
                        direction = Direction::WEST;
                    },
                    Direction::WEST => {
                        direction = Direction::NORTH;
                    },
                }
            },
        }

        // Move in each direction, one step at a time, saving each location to history
        // If we've been somewhere before, then save the first occurrence of the double
        if debug { println!("Moving {} units {:?}", instruction.distance, direction); }
        match direction {
            Direction::NORTH => {
                //y += instruction.distance as isize;
                for _ in 0..instruction.distance {
                    y += 1 as isize;
                    let addr_tuple = (x, y);
                    if history.contains(&addr_tuple) && first_double_visit == None {
                        if debug { println!("We've been here before!"); }
                        first_double_visit = Some(addr_tuple);
                    } else if first_double_visit == None {
                        history.insert(addr_tuple);
                    }
                }
            },
            Direction::EAST => {
                //x += instruction.distance as isize;
                for _ in 0..instruction.distance {
                    x += 1 as isize;
                    let addr_tuple = (x, y);
                    if history.contains(&addr_tuple) && first_double_visit == None {
                        if debug { println!("We've been here before!"); }
                        first_double_visit = Some(addr_tuple);
                    } else if first_double_visit == None {
                        history.insert(addr_tuple);
                    }
                }
            },
            Direction::SOUTH => {
                //y -= instruction.distance as isize;
                for _ in 0..instruction.distance {
                    y -= 1 as isize;
                    let addr_tuple = (x, y);
                    if history.contains(&addr_tuple) && first_double_visit == None {
                        if debug { println!("We've been here before!"); }
                        first_double_visit = Some(addr_tuple);
                    } else if first_double_visit == None {
                        history.insert(addr_tuple);
                    }
                }
            },
            Direction::WEST => {
                //x -= instruction.distance as isize;
                for _ in 0..instruction.distance {
                    x -= 1 as isize;
                    let addr_tuple = (x, y);
                    if history.contains(&addr_tuple) && first_double_visit == None {
                        if debug { println!("We've been here before!"); }
                        first_double_visit = Some(addr_tuple);
                    } else if first_double_visit == None {
                        history.insert(addr_tuple);
                    }
                }
            },
        }
    }

    println!("Finished at {},{} facing {:?}", x, y, direction);
    let total_distance = x.abs() + y.abs();
    println!("Total distance away from start is {}", total_distance);
    if let Some(fdc) = first_double_visit {
        println!("First double visited location is {},{}", fdc.0, fdc.1);
        let fdc_distance = fdc.0.abs() + fdc.1.abs();
        println!("First double visited location distance away from start is {}", fdc_distance);
    }
}

fn parse_instructions(
    _debug: bool,
    input: &str,
) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();

    // Get instructions from each line. They should be all on one line,
    // but we will check all ust in case.
    for line in input.lines() {
        // Trim whitespace and skip empty lines.
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Split each line on commas to get each instruction in a Vec<&str>
        let instructs: Vec<&str> = line.split(",").collect();

        // For each instruction, check that it starts with R or L, chop that off,
        // and convert the rest to a number.
        for instruct in instructs {
            // Remove whitespace
            let instruct = instruct.trim();

            // We either start with R or L, or are invalid
            let turn: Turn = if instruct.starts_with("R") {
                Turn::RIGHT
            } else if instruct.starts_with("L") {
                Turn::LEFT
            } else {
                eprintln!("Invalid instruction encountered! {}", instruct);
                continue;
            };

            // Convert remaining to usize for distance
            let distance = match instruct.replace("R", "").replace("L", "").parse::<usize>() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("Invalid instruction encountered! {}", instruct);
                    continue;
                }
            };

            let instruction = Instruction{ turn, distance };

            instructions.push(instruction);
        }
    }

    instructions
}

#[derive(Debug)]
struct Instruction {
    turn: Turn,
    distance: usize,
}
#[derive(Debug)]
enum Turn {
    LEFT,
    RIGHT,
}
#[derive(Debug)]
enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}