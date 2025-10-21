use shared::*;
use clap::Parser;
use std::process;

/// Advent Of Code 2015 Day 24 Part 1 and 2 solution
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input file containing package weights
    #[arg(short, long)]
    filename: String,

    /// The number of groups to split the packages into
    #[arg(short, long, default_value_t = 3,)]
    groups: usize,

    /// Toggle debug messages
    #[arg(long, default_value_t = false)]
    debug: bool,
}

fn main() {
    // Parse args
    let args = Args::parse();

    // Check for valid input
    if args.groups < 2 {
        eprintln!("Cannot split into {} groups!", args.groups);
        process::exit(1);
    }

    // Load input file and parse. Sort largest to smallest (larger packages more likely to result in smaller groups)
    let file_contents = load_input_file(&args.filename);
    let mut package_weights: Vec<usize> = file_contents
        .lines()
        .map(|line| line.trim().parse::<usize>().unwrap_or(0))
        .collect();
    package_weights.sort_by(|a, b| b.cmp(a));

    // Count packages and get total sum
    let total_count: usize = package_weights.len();
    if args.debug { println!("Total package count: {}", total_count); }
    if total_count == 0 {
        eprintln!("There are no packages!");
        process::exit(1);
    }
    let total_sum: usize = package_weights.iter().sum();
    if args.debug { println!("Total package weights: {}", total_sum); }

    // Confirm possibility.  Total sum must be divisible by number of groups.
    if total_sum % args.groups > 0 {
        eprintln!("These packages cannot be split into {} groups!", args.groups);
        process::exit(1);
    }

    // Create groups
    let mut groups: Vec<Vec<usize>> = vec![];
    for _ in 0..args.groups {
        groups.push(vec![]);
    }

    let target_sum: usize = total_sum / args.groups;
    if args.debug { println!("Target group sum: {}", target_sum); }

    // The maximum group size should be smaller than 1/n of the total packages
    let max_size: usize = total_count.div_ceil(args.groups);
    if args.debug { println!("Max group size: {}", max_size); }

    let mut smallest_group = usize::MAX;
    let mut best_qe = usize::MAX;

    find_smallest_group(args.debug, &target_sum, &max_size, package_weights, vec![], &mut smallest_group, &mut best_qe);

    if best_qe == usize::MAX {
        println!("No valid grouping found.");
        process::exit(1);
    } else {
        println!("Best QE found: {}", best_qe);
    }
}

// Recursively try all possible combinations of packages to find the smallest group
// that makes the target sum.
fn find_smallest_group(
    debug: bool,
    target_sum: &usize,
    max_size: &usize,
    remaining_package_weights: Vec<usize>,
    current_group: Vec<usize>,
    smallest_group: &mut usize,
    best_qe: &mut usize,
) {

    if debug { println!("Remaining packages: {:?}", remaining_package_weights); }

    // Loop over each possible package weights
    for weight in remaining_package_weights.iter() {

        let mut current_group_clone = current_group.clone();
        let mut remaining_package_weights_clone = remaining_package_weights.clone();

        // Try adding the current weight to the current group
        if debug { println!("Trying to add {} to {:?}", weight, current_group_clone); }
        current_group_clone.push(*weight);

        // If this pushes us over the target weight, or if we've already found a smaller group, skip to the next package.
        let sum: usize = current_group_clone.iter().sum();
        let count: usize = current_group_clone.len();
        if sum > *target_sum || count > *smallest_group || count > *max_size {
            if debug { println!("Adding {} to {:?} makes it too big! Skipping...", weight, current_group); }
            continue;
        } else if sum == *target_sum {
            if debug { println!("Found valid group! {:?}", current_group_clone); }

            // If the valid group is smaller than the current smallest group it wins
            // If they're equal, then the smallest QE group wins
            let qe: usize = current_group_clone.iter().product();
            if count < *smallest_group {
                if debug { println!("New group is smaller than {}", smallest_group); }
                *smallest_group = count;
                *best_qe = qe;
            } else if count == *smallest_group {
                if debug { println!("New group is as small as {}", smallest_group); }
                if qe < *best_qe {
                    if debug { println!("New QE is better than {}", best_qe); }
                    *smallest_group = count;
                    *best_qe = qe;
                }
            } else {
                if debug { println!("New group is bigger than {}", smallest_group); }
            }
            continue;
        }

        // Remove current weight from remeining weights
        remaining_package_weights_clone.retain(|&w| w != *weight);

        find_smallest_group(debug, target_sum, max_size, remaining_package_weights_clone.clone(), current_group_clone.clone(), smallest_group, best_qe);
    }
}