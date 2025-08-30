use shared::*;
use std::{
    env,
    process,
};

fn main() {
    let args = parse_args_iterations();
    let mut debug: bool = false;
    if (args.len() == 3 && &args[2] == "--debug") || (args.len() == 4 && (&args[2] == "--debug" || &args[3] == "--debug")) {
        debug = true;
    }
    
    let file_contents = load_input_file(&args[1]);

    if debug { println!("{file_contents}"); }

    let iterations:usize = match args.len() {
        3 if args[2].parse::<usize>().is_ok() => args[2].parse::<usize>().unwrap(),
        4 if args[2].parse::<usize>().is_ok() || args[3].parse::<usize>().is_ok() => {
            if args[2].parse::<usize>().is_ok() {
                args[2].parse::<usize>().unwrap()
            } else {
                args[3].parse::<usize>().unwrap()
            }
        },
        _ => 1,
    };

    let mut result = file_contents.clone();
    
    for _ in 0..iterations {
        result = see_and_say(debug, &result);
    }

    println!("After {iterations} iteration(s) the answer is: {}", result.len());
}

fn see_and_say(debug:bool, input: &str) -> String {
    let input = input.trim();
    // At the start of the string the last char is irrelevant - lets use a null-byte
    let mut last_char = '\0';
    let mut char_count: usize = 0; // How many of the same char have we seen

    let mut result: String = String::new();

    for char in input.chars() {
        if debug { println!("Found a {char}"); }
        if char == last_char {
            char_count += 1;
            if debug { println!("Seen {char_count} {char}"); }
        } else {
            // Do not store the null byte we start with
            if last_char != '\0' {
                if debug { println!("Recording {char_count} {last_char}"); }
                result.push_str(&char_count.to_string());
                result.push(last_char);
            }

            last_char = char;
            char_count = 1;
        }
    }
    // record the final char
    if last_char != '\0' {
        if debug { println!("Recording {char_count} {last_char}"); }
        result.push_str(&char_count.to_string());
        result.push(last_char);
    }

    if debug { println!("{result}"); }
    result
}