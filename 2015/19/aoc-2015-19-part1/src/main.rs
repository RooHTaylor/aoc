use shared::*;
use std::collections::{HashMap, HashSet};

fn main() {
    let args = parse_args();
    let mut debug: bool = false;
    if args.len() == 3 && &args[2] == "--debug" {
        debug = true;
    }
    
    let file_contents = load_input_file(&args[1]);
    if debug { println!("{file_contents}"); }

    let (initial_string, transformations) = parse_input(&file_contents);
    if debug { println!("{}\n{:#?}", initial_string, transformations); }
    
    let molecules = create_molecules(&initial_string, &transformations);

    println!("There are {} unique molecules.", molecules.len());
}

// For each possible transformation, find the position within the string, and
// replace the value with each new value.  Each created value is a new molecule
fn create_molecules(input: &str, transformations: &HashMap<String, Vec<String>>) -> HashSet<String> {
    let mut molecules: HashSet<String> = HashSet::new();

    for (k, v) in transformations {
        let mut position = 0;

        while let Some(index) = input[position..].find(k) {
            let actual_index = position + index;

            for value in v {
                // Create a new string with the key replaced by the value
                let mut replaced = input.to_string();
                replaced.replace_range(actual_index..actual_index + k.len(), value);
                molecules.insert(replaced);
            }

            position = actual_index + 1;
        }
    }

    molecules
}

// Parse input file contents line by line.
// Transformation lines should be in the format string => string
// Or just the string for the initial value. If => is not in the line, the initial
// value is set to that line
fn parse_input(input: &str) -> (String, HashMap<String, Vec<String>>) {
    let mut initial_string = String::new();
    let mut transformations: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.lines() {
        if line.trim().is_empty() { continue; }

        let parts = match line.trim().split_once("=>") {
            Some(p) => p,
            None => {
                initial_string = line.trim().to_string();
                continue;
            }
        };

        transformations.entry(parts.0.trim().to_string())
            .or_insert_with(Vec::new) // Insert a new Vec<String> if the key doesn't exist
            .push(parts.1.trim().to_string());
    }

    (initial_string, transformations)
}