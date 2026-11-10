mod days;

use aoc_common::{Options, read_input};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Which day to run
    #[arg(short, long)]
    day: u8,

    /// Which part to run (1 or 2) - ommit to run both
    #[arg(short, long)]
    part: Option<u8>,

    /// Path to the puzzle input ("-" for stdin).
    /// Defaults to inputs/dayNN.txt
    #[arg(short, long)]
    input: Option<PathBuf>,

    /// Print extra debugging output
    #[arg(long)]
    debug: bool,

    /// Number of iterations (for days that need it)
    #[arg(long)]
    iterations: Option<u32>,
}

fn main() {
    // Parse command line arguments
    let cli = Cli::parse();

    // Check for a provided part. If we have one, use it. Otherwise try to run
    // both parts.
    let mut parts = Vec::with_capacity(2);
    if let Some(part) = cli.part {
        parts.push(part);
    } else {
        parts.push(1);
        parts.push(2);
    }

    // Prepare the options to pass to the solver
    let opts = Options{
        debug: cli.debug,
        iterations: cli.iterations,
    };

    let path = cli
        .input
        .unwrap_or_else(|| PathBuf::from(format!("inputs/day{:02}.txt", cli.day)));

    let input = match read_input(&path) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("note: couldn't read {}: {e} (using empty input)", path.display());
            String::new()
        }
    };

    for part in parts {
        let Some(solver) = days::solver(cli.day, part) else {
            eprintln!("Day {} part {} is not implemented yet", cli.day, part);
            std::process::exit(1);
        };

        match solver(&input, &opts) {
            Ok(answer) => println!("{answer}"),
            Err(e) => {
                eprintln!("[ERROR]: {e}");
                std::process::exit(1);
            }
        }
    }
}
