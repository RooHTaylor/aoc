use shared::*;
use clap::Parser;
use std::process;

/// Advent Of Code 2015 Day 23 Part 1 solution
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input file containing instructions
    #[arg(short, long)]
    filename: String,

    /// Starting a value
    #[arg(short)]
    a: Option<usize>,

    /// Starting b value
    #[arg(short)]
    b: Option<usize>,

    /// Toggle debug messages
    #[arg(long, default_value_t = false)]
    debug: bool,
}

fn main() {
    // Parse args
    let args = Args::parse();

    // Load input file
    let file_contents = load_input_file(&args.filename);

    let mut computer = Computer::new();
    if let Some(sa) = args.a {
        computer.a = sa;
    }
    if let Some(sb) = args.b {
        computer.b = sb;
    }

    let instructions: Vec<&str> = file_contents
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect();

    let mut current_instruction: usize = 0;
    loop {

        if current_instruction >= instructions.len() {
            eprintln!("Instruction pointer out of scope!");
            break;
        }

        if args.debug { println!("i:{current_instruction}, a:{} b:{}", computer.a, computer.b); }

        current_instruction = execute_instruction(args.debug, &mut computer, current_instruction, &instructions)

    }

    println!("Register a: {}, Register b: {}", computer.a, computer.b);
}

fn execute_instruction(debug: bool, computer: &mut Computer, current_instruction: usize, instructions: &Vec<&str>) -> usize {

    let parts: Vec<&str> = instructions[current_instruction].split(" ").collect();
    if parts.len() < 2 {
        eprintln!("Malformed instruction! {}", instructions[current_instruction]);
        process::exit(1);
    }

    let mut next_instruction = current_instruction + 1;
    match parts[0].trim() {
        "hlf" => {
            
            match parts[1] {
                "a" => {
                    if debug { println!("halving a"); }
                    computer.a = computer.a / 2;
                },
                "b" => {
                    if debug { println!("halving b"); }
                    computer.b = computer.b / 2;
                },
                _ => {
                    eprintln!("Register does not exist! {}", parts[1]);
                    process::exit(1);
                }
            }
        },
        "tpl" => {
            match parts[1] {
                "a" => {
                    if debug { println!("trippling a"); }
                    computer.a = computer.a * 3;
                },
                "b" => {
                    if debug { println!("trippling b"); }
                    computer.b = computer.b * 3;
                },
                _ => {
                    eprintln!("Register does not exist! {}", parts[1]);
                    process::exit(1);
                }
            }
        },
        "inc" => {
            match parts[1] {
                "a" => {
                    if debug { println!("incrementing a"); }
                    computer.a += 1;
                },
                "b" => {
                    if debug { println!("incrementing b"); }
                    computer.b += 1;
                },
                _ => {
                    eprintln!("Register does not exist! {}", parts[1]);
                    process::exit(1);
                }
            }
        },
        "jmp" => {
            next_instruction = match parts[1].parse::<isize>() {
                Ok(n) => {
                    if n < 0 {
                        match current_instruction.checked_sub(n.abs() as usize) {
                            Some(sn) => sn,
                            None => {
                                eprintln!("Instruction overflow! {}", parts[1]);
                                process::exit(1);
                            },
                        }
                    } else {
                        match current_instruction.checked_add(n as usize) {
                            Some(an) => an,
                            None => {
                                eprintln!("Instruction overflow! {}", parts[1]);
                                process::exit(1);
                            },
                        }
                    }
                },
                Err(_) => {
                    eprintln!("Invalid instruction offset! {}", parts[1]);
                    process::exit(1);
                }
            };
            if debug { println!("jumping to {}", next_instruction); }
        },
        "jie" => {
            match parts[1].replace(",", "").as_str() {
                "a" => {
                    if computer.a % 2 != 0 {
                        if debug { println!("a is not even "); }
                        return next_instruction;
                    }
                    if debug { print!("a is even "); }
                },
                "b" => {
                    if computer.b % 2 != 0 {
                        if debug { println!("b is not even "); }
                        return next_instruction;
                    }
                    if debug { print!("b is even "); }
                },
                _ => {
                    eprintln!("Register does not exist! {}", parts[1]);
                    process::exit(1);
                }
            }

            if parts.len() != 3 {
                eprintln!("Malformed instruction! {}", instructions[current_instruction]);
                process::exit(1);
            }

            next_instruction = match parts[2].parse::<isize>() {
                Ok(n) => {
                    if n < 0 {
                        match current_instruction.checked_sub(n.abs() as usize) {
                            Some(sn) => sn,
                            None => {
                                eprintln!("Instruction overflow! {}", parts[1]);
                                process::exit(1);
                            },
                        }
                    } else {
                        match current_instruction.checked_add(n as usize) {
                            Some(an) => an,
                            None => {
                                eprintln!("Instruction overflow! {}", parts[1]);
                                process::exit(1);
                            },
                        }
                    }
                },
                Err(_) => {
                    eprintln!("Invalid instruction offset! {}", parts[1]);
                    process::exit(1);
                }
            };
            if debug { println!("jumping to {}", next_instruction); }
        },
        "jio" => {
            match parts[1].replace(",", "").as_str() {
                "a" => {
                    if computer.a != 1 {
                        if debug { println!("a is not one "); }
                        return next_instruction;
                    }
                    if debug { print!("a is one "); }
                },
                "b" => {
                    if computer.b != 1 {
                        if debug { println!("b is not one "); }
                        return next_instruction;
                    }
                    if debug { print!("b is one "); }
                },
                _ => {
                    eprintln!("Register does not exist! {}", parts[1]);
                    process::exit(1);
                }
            }

            if parts.len() != 3 {
                eprintln!("Malformed instruction! {}", instructions[current_instruction]);
                process::exit(1);
            }

            next_instruction = match parts[2].parse::<isize>() {
                Ok(n) => {
                    if n < 0 {
                        match current_instruction.checked_sub(n.abs() as usize) {
                            Some(sn) => sn,
                            None => {
                                eprintln!("Instruction overflow! {}", parts[1]);
                                process::exit(1);
                            },
                        }
                    } else {
                        match current_instruction.checked_add(n as usize) {
                            Some(an) => an,
                            None => {
                                eprintln!("Instruction overflow! {}", parts[1]);
                                process::exit(1);
                            },
                        }
                    }
                },
                Err(_) => {
                    eprintln!("Invalid instruction offset! {}", parts[1]);
                    process::exit(1);
                }
            };
            if debug { println!("jumping to {}", next_instruction); }
        },
        _ => {
            println!("Malformed instruction! {}", instructions[current_instruction]);
            process::exit(1);
        }
    }
    
    next_instruction
}

struct Computer {
    a: usize,
    b: usize,
}

impl Computer {
    fn new() -> Computer {
        let a: usize = 0;
        let b: usize = 0;

        Computer{ a, b }
    }
}