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

    for line in lines.map_while(Result::ok) {

        // skip empty lines
        if line.is_empty() {
            continue;
        }

        let mut any_two_chars = false;
        let mut repeat_with_middle = false;
        let chars: Vec<char> = line.trim().chars().collect();

        for i in 0..chars.len() {

            if i > 1 && !repeat_with_middle {
                if chars[i] == chars[i - 2] {
                    repeat_with_middle = true;
                }
            }

            if i > 0 {
                let mut double_char = String::new();
                double_char.push(chars[i - 1]);
                double_char.push(chars[i]);

                let matches: Vec<_> = line.match_indices(double_char.as_str()).collect();
                if matches.len() < 2 {
                    continue;
                }

                let mut last_match_index: usize = 0;
                for m in matches {
                    let (index, _) = m;
                    // ignore match unless the index is > the current i and 
                    // last match index + 2 (ie not this match and not overlapping 
                    // the last match
                    if index <= i || index <= last_match_index + 1 {
                        last_match_index = index;
                        continue;
                    }
                    
                    // If we're here, we've found at least one match that isn't
                    // the current position, and isn't overlapping the current
                    // position
                    any_two_chars = true;
                    break;
                }
            }
        }

        if any_two_chars && repeat_with_middle {
            nice_strings += 1;
        }
    }

    println!("Found {} nice strings!", nice_strings);
}
