use shared::*;
use std::collections::HashMap;

fn main() {
    let args = parse_args();
    
    let file_contents = load_input_file(&args[1]);

    let num_nice_strings = count_nice_strings(&file_contents);

    println!("The total number of nice strings is {num_nice_strings}");
}

fn count_nice_strings(contents: &str) -> usize {

    let mut nice_count: usize = 0;

    for line in contents.lines() {
        // Skip empty lines
        if line.trim().is_empty() { continue; }

        if is_nice(&line) {
            nice_count += 1;
        }
    }

    nice_count
}

// Evaluate if a string is nice or not
fn is_nice(input_string: &str) -> bool {

    if input_string.trim().is_empty() {
        return false
    }

    let input_string_chars: Vec<char> = input_string.trim().chars().collect();

    let mut double_letters: HashMap<String, (usize, usize)> = HashMap::new();

    let mut repeating_chars: bool = false;
    let mut pair_of_pairs: bool = false;

    for (i, c) in input_string_chars.iter().enumerate() {
        // Cannot perform evaluations on the first char.
        if i == 0 {
            continue;
        }

        let mut last_two = String::with_capacity(2);
        last_two.push(input_string_chars[i-1]);
        last_two.push(*c);

        // Check if we've encountered these two letters before by searching for 
        // them in the HashMap.  If we have seen them, confirm that the indicies 
        // do not overlap.
        // If they're new, store them in the HashMap with where we found them
        match double_letters.get(&last_two) {
            Some((pi1, pi2)) => {
                if *pi1 != i && *pi1 != i-1 && *pi2 != i && *pi2 != i-1 {
                    pair_of_pairs = true;
                }
            },
            None => {
                double_letters.insert(last_two, (i-1, i));
            }
        }

        if i >= 2 && input_string_chars[i-2] == *c {
            repeating_chars = true;
        }
    }

    repeating_chars && pair_of_pairs
}