use shared::*;
use std::collections::{HashMap, HashSet};
use md5;

fn main() {
    let args = parse_args();
    let mut debug: bool = false;
    if args.len() == 3 && &args[2] == "--debug" {
        debug = true;
    }
    
    let file_contents = load_input_file(&args[1]);
    if debug { println!("{file_contents}"); }

    let (target_molecule, transformations, sorted_reverse_keys) = parse_input(&file_contents);
    if debug { println!("{}\n{:#?}\n{:#?}", target_molecule, transformations, sorted_reverse_keys); }

    let smallest_molecule = "e".to_string();
    let mut seen_molecules: HashSet<md5::Digest> = HashSet::new();

    let mut min_steps: usize = usize::MAX;

    new_molecules(debug, target_molecule, &transformations, 1, &smallest_molecule, &mut min_steps, &mut seen_molecules);

    if min_steps == usize::MAX {
        println!("We could not create the target molecule.");
    } else {
        println!("It took {} steps to find the target molecule.", min_steps);
    }
}

// for each possible transformation, create a new molecule and step into that
// molecule if we haven't seen it yet.
fn new_molecules(
    debug: bool,
    input: String,
    transformations: &HashMap<String, Vec<String>>,
    step: usize,
    target: &str,
    min_steps: &mut usize,
    seen_molecules: &mut HashSet<md5::Digest>
) {

    if debug { println!("Step: {step} - {input}"); }

    // no point checking beyond what we've already decided is the min
    if step > *min_steps {
        if debug { println!("We've already found a solution smaller than this - Skipping..."); }
        return;
    }

    let num_es = input.chars().filter(|&c| c == 'e').count();
    if num_es > 1 || (num_es == 1 && input.len() > 1) {
        if debug { println!("There are too many \"e\"s - Skipping..."); }
        return;
    }

    for (k, v) in transformations {

        //if debug { println!("Searching for {k}"); }

        // Search for the key, and skip to the next if we don't find
        let index = match input.find(k) {
            Some(n) => n,
            None => {
                continue;
            }
        };

        if debug { println!("Found {k} at position {index}"); }

        // For each of the possible replacement values, make a new molecule
        for value in v {
            // Create a new string with the key replaced by the value
            let mut replaced = input.to_string();
            replaced.replace_range(index..index + k.len(), value);

            let digest = md5::compute(replaced.clone());
            if seen_molecules.contains(&digest) {
                if debug { println!("Duplicate molecule."); }
                continue;
            } else {
                seen_molecules.insert(digest);
            }

            // If we've found the target, update the min_steps if needed and continue.
            if replaced.as_str() == target {
                if debug { println!("Target Found!!!"); }
                if step < *min_steps {
                    println!("New record of {step} steps");
                    *min_steps = step;
                }
                continue;
            }

            // Step into the next iteration with the newly generated molecule
            new_molecules(debug, replaced, transformations, step + 1, target, min_steps, seen_molecules);
        }
    }

}

// Parse input file contents line by line.
// Transformation lines should be in the format string => string
// Or just the string for the target value. If => is not in the line, the target
// value is set to that line
fn parse_input(input: &str) -> (String, HashMap<String, Vec<String>>, Vec<String>) {
    let mut target_molecule = String::new();
    let mut transformations: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.lines() {
        if line.trim().is_empty() { continue; }

        let parts = match line.trim().split_once("=>") {
            Some(p) => p,
            None => {
                target_molecule = line.trim().to_string();
                continue;
            }
        };

        transformations.entry(parts.1.trim().to_string())
            .or_insert_with(Vec::new) // Insert a new Vec<String> if the key doesn't exist
            .push(parts.0.trim().to_string());
    }

    let mut sorted_reverse_keys: Vec<String> = transformations.keys().cloned().collect();
    sorted_reverse_keys.sort_by(|a, b| b.len().cmp(&a.len()));

    (target_molecule, transformations, sorted_reverse_keys)
}