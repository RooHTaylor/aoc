use std::env;
use std::process;
use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

fn main() {
    // Get the arguments
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <inputfilename>", &args[0]);
        process::exit(1);
    }
    let filename = Path::new(&args[1]);

    let file = match File::open(filename) {
        Ok(fp) => fp,
        Err(_) => {
            println!("Unable to open file!");
            process::exit(2);
        },
    };
    let lines = io::BufReader::new(file).lines();

    let mut nice_strings: usize = 0;

    'lines: for line in lines.map_while(Result::ok) {

        // skip empty lines
        if line.trim().len() < 1 {
            continue;
        }

        // we need to keep track of number of vowels per line and
        // previous char (for 2x same char in a row and bad strings)
        let mut vowel_count: usize = 0;
        let mut prev = String::new();
        let mut double_chars = false;

        for c in line.trim().chars() {

            // Check if c matches any of the prohibited strings when paired with
            // previous char. Skip to the next line if found
            let mut prev2 = prev.clone();
            prev2.push(c);
            match prev2.as_str() {
                "ab" | "cd" | "pq" | "xy" => {
                    continue 'lines;
                },
                _ => (),
            }

            // Check if current char == prev char before changing prev
            if c.to_string() == prev && !double_chars {
                double_chars = true;
            }

            // If char is a vowel, increment vowel_count
            match c {
                'a' | 'e' | 'i' | 'o' | 'u' => {
                    vowel_count += 1;
                },
                _ => (),
            }

            prev = c.to_string();
        }

        // check 2 conditions. will not reach here if string had the bad strings
        // in it, so only check the other two conditions.
        if vowel_count >= 3 && double_chars {
            nice_strings += 1;
        }
    }

    println!("Found {} nice strings!", nice_strings);
}
