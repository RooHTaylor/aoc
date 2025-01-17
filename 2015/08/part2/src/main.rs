use std::env;
use std::process;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    // read in file from args and open
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <inputfilename>", args[0]);
        process::exit(1);
    }
    let filename = Path::new(&args[1]);
    let file = match File::open(filename) {
        Ok(fp) => fp,
        Err(_) => {
            println!("Unable to open file! {}", &args[1]);
            process::exit(2);
        },
    };
    let lines = io::BufReader::new(file).lines();

    let mut start_length: usize = 0;
    let mut new_length: usize = 0;

    for line in lines.map_while(Result::ok) {
        // skip empty lines
        if line.is_empty() {
            continue;
        }

        start_length += line.trim().len();
        new_length += line.trim().len();
        // Automatically add 2 for the new start and end "
        new_length += 2;

        // Loop over each char. If we find an escapable char, escape it
        for c in line.trim().chars() {
            if c == '\\' {
                // escape all \
                new_length += 1;
                continue;
            } else if c == '"' {
                new_length += 1;
                continue;
            } else {
                continue;
            }
        }
    }

    println!("Initial length: {}", start_length);
    println!("Encoded length: {}", new_length);

    let answer = new_length - start_length;
    println!("Answer: {}", answer);
}
