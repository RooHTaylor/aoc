use std::env;
use std::process;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    // Read filename from args
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <inputfilename>", &args[0]);
        process::exit(1);
    }
    let filename = Path::new(&args[1]);
    // Open file and read lines into a buffered reader
    let file = match File::open(&filename) {
        Ok(fp) => fp,
        Err(_) => {
            println!("Unable to open file! {}", filename.to_str().unwrap());
            process::exit(2);
        },
    };
    let lines = io::BufReader::new(file).lines();

    let mut sum: isize = 0;

    for line in lines.map_while(Result::ok) {
        // ignore empty lines
        if line.is_empty() {
            continue;
        }

        let mut in_string: bool = false;
        let mut string_buf: String = String::new();

        let mut in_number: bool = false;
        let mut number_buf: String = String::new();

        let mut indent: String = String::new();

        // Loop line char by char.
        for c in line.chars() {
            match c {
                '{' => {
                    println!("{}Object", indent);
                    indent.push_str("  ");
                },
                '}' => {
                    if in_number {
                        in_number = false;
                        println!("{}{}", indent, number_buf);
                        let num: isize = match number_buf.parse() {
                            Ok(n) => n,
                            Err(_) => {
                                println!("We found {} but could not parse", number_buf);
                                process::exit(5);
                            }
                        };
                        sum += num;
                        number_buf.clear();
                    }
                    indent.truncate(indent.len() - 2);
                    println!("{}End object", indent);
                },
                '[' => {
                    println!("{}Array", indent);
                    indent.push_str("  ");
                },
                ']' => {
                    if in_number {
                        in_number = false;
                        println!("{}{}", indent, number_buf);
                        let num: isize = match number_buf.parse() {
                            Ok(n) => n,
                            Err(_) => {
                                println!("We found {} but could not parse", number_buf);
                                process::exit(5);
                            }
                        };
                        sum += num;
                        number_buf.clear();
                    }
                    indent.truncate(indent.len() - 2);
                    println!("{}End array", indent);
                },
                ':' => {
                    println!("{}is", indent);
                },
                '"' => {
                    if in_string {
                        println!("{}\"{}\"", indent, &string_buf);
                        string_buf.clear();
                        in_string = false;
                    } else {
                        in_string = true;
                    }
                },
                ',' => {
                    println!("{}and", indent);
                    if in_number {
                        in_number = false;
                        println!("{}{}", indent, number_buf);
                        let num: isize = match number_buf.parse() {
                            Ok(n) => n,
                            Err(_) => {
                                println!("We found {} but could not parse", number_buf);
                                process::exit(5);
                            }
                        };
                        sum += num;
                        number_buf.clear();
                    }
                },
                '-' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {
                    if !in_string {
                        if !in_number {
                            in_number = true;
                        }
                        number_buf.push(c);
                    }
                },
                _ => {
                    if in_string {
                        string_buf.push(c);
                    }

                    if in_number {
                        in_number = false;
                        println!("{}{}", indent, number_buf);
                        let num: isize = match number_buf.parse() {
                            Ok(n) => n,
                            Err(_) => {
                                println!("We found {} but could not parse", number_buf);
                                process::exit(5);
                            }
                        };
                        sum += num;
                        number_buf.clear();
                    }
                }
            }
        }
    }

    println!("Sum is {}", sum);
}
