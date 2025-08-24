use shared::*;
use std::{
    env,
    process,
};

fn main() {
    let args = custom_parse_args();
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

    let forbidden_chars: Vec<char> = vec!['i', 'l', 'o'];
    let mut result = file_contents.clone();
    //let mut result = String::new();
    
    for n in 1..=iterations {
        while !validate_password(debug, &result, &forbidden_chars) {
            result = count_with_chars(debug, &result, &forbidden_chars);
        }
        println!("After {n} iteration(s) the answer is: {}", result);
        // increment by 1 before next itter
        result = count_with_chars(debug, &result, &forbidden_chars);
    }
}

fn validate_password(debug: bool, input: &str, forbidden_chars: &Vec<char>) -> bool {

    let char_list: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

    let mut char_indicies: Vec<usize> = Vec::new();

    for ic in input.chars() {
        if forbidden_chars.contains(&ic) {
            return false;
        }
        let ci: usize = match char_list.iter().position(|&c| c == ic) {
            Some(n) => n,
            None => {
                eprintln!("An invalid char was encountered!");
                return false;
            }
        };
        char_indicies.push(ci);
    }
    char_indicies.reverse();

    let mut double_char_locations: Vec<usize> = Vec::new();
    let mut char_run: bool = false;

    for (i, _p) in char_indicies.iter().enumerate() {
        if 
        i >= 1 // is it possible to have seen double chars yet?
        && (char_indicies[i-1] == char_indicies[i]) // Are the chars the same?
        && double_char_locations.len() < 4 // Are we still looking for double chars?
        && (!double_char_locations.contains(&(i-1)) && !double_char_locations.contains(&i)) // Have we not seen these before?
        {
            // These are double chars. Save them.
            if debug { println!("Char {} and {} are the same. Saving", i-1, i); }
            double_char_locations.push(i-1);
            double_char_locations.push(i);
        }

        // Are the last 3 chars incremented by 1?
        if 
        i >= 2 // is it possible to have seen three chars yet?
        && !char_run // Are we still looking for a run?
        {
            if let (Some(second_last), Some(last)) = (
                char_indicies[i-2].checked_sub(2),
                char_indicies[i-1].checked_sub(1)
            ) {
                if second_last == last && last == char_indicies[i] {
                    // This is a run.
                    char_run = true;
                }
            }
        }
    }

    double_char_locations.len() == 4 && char_run && input.len() == 8
}

fn count_with_chars(debug: bool, input: &str, forbidden_chars: &Vec<char>) -> String {
    let char_list: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

    let mut input: Vec<char> = input.chars().collect();

    // If the input is empty, push the first char and return
    if input.len() == 0 {
        if debug { println!("Empty input. Starting with {}", &char_list[0]); }
        input.push(char_list[0]);
        let result: String = input.iter().collect();
        return result;
    }
    if debug { println!("Input: {}", input.iter().collect::<String>()); }

    // We're adding 1, so there is a "carry" by default
    let mut carry: bool = true;
    // Start with the last char
    let mut i: usize = input.len() - 1;
    let mut count = 0;
    // As long as there is still a carry and we're not at the end of the string,
    // continue incrementing chars at the next index. We need to count the chars
    // processed here, since usize >= 0 is always true, so we can't use the index
    // countdown to know when we're at the end of the list.
    while carry && count < input.len() {
        let mut next_index = match char_list.iter().position(|&c| c == input[i]) {
            Some(n) => {
                if n + 1 >= char_list.len() {
                    n + 1 - char_list.len()
                } else {
                    carry = false;
                    n + 1
                }
            },
            None => {
                eprintln!("An invalid char was encountered!");
                process::exit(1);
            }
        };
        if debug { println!("At index {}, {} becomes {}", i, input[i], char_list[next_index]); }
        if forbidden_chars.contains(&char_list[next_index]) {
            if debug { println!("This char is listed as forbidden! Picking the next one."); }
            next_index = match char_list.iter().position(|&c| c == input[i]) {
                Some(n) => {
                    if n + 1 >= char_list.len() {
                        n + 1 - char_list.len()
                    } else {
                        carry = false;
                        n + 1
                    }
                },
                None => {
                    eprintln!("An invalid char was encountered!");
                    process::exit(1);
                }
            };
        }
        input[i] = char_list[next_index];

        // Don't decrement i below 0 since it's usize
        if i > 0 {
            i -= 1;
        }
        count += 1;
    }
    // If carry is still set then we need to add a "digit" to the front.
    if carry {
        input = {
            let mut t = vec!['a'];
            t.append(&mut input);
            t
        };
        if debug { println!("Remaining carry after all characters processed."); }
    }

    let result: String = input.iter().collect();
    if debug { println!("Result: {}", result); }
    result
}

// Read the arguments and confirm an input file was provided or the number of iterations was provided
// The function allows for an optional 3rd argument, --debug
// 
// # Exits - code 1
//
// This function will exit the process if the number of arguments are incorrect
pub fn custom_parse_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();

    if
        args.len() < 2
        || (args.len() == 3 && !(&args[2] == "--debug" || args[2].parse::<usize>().is_ok())) 
        || (args.len() == 4 && !(&args[2] == "--debug" || args[2].parse::<usize>().is_ok()) && !(&args[3] == "--debug" || args[3].parse::<usize>().is_ok()))
    {
        eprintln!("Usage: {} <input file> [<iterations>] [--debug]", &args[0]);
        eprintln!("       OR cargo run -- <input file> [iterations] [--debug]");
        process::exit(1);
    }

    args
}