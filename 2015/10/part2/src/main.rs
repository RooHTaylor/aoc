use std::env;
use std::process;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    // Read filename from args
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <inputfilename>", &args[0]);
        process::exit(1);
    }
    let filename = Path::new(&args[1]);
    // Open file and read lines into a buffered reader
    let file = match File::open(&filename) {
        Ok(fp) => fp,
        Err(_) => {
            println!("Unable to open file! {}", filename.to_str().unwrap());
            process::exit(2);
        },
    };
    let lines = io::BufReader::new(file).lines();

    let mut input: String = String::new();

    for line in lines.map_while(Result::ok) {
        // ignore blank lines
        if line.is_empty() {
            continue;
        }

        // Our input should only be one line
        input.push_str(line.trim());
        break;
    }

    //println!("Input is: {}", input);

    if input.is_empty() {
        println!("Input was empty!");
        process::exit(3);
    }

    let max_i: usize = 50;
    for i in 1..=max_i {
        let mut new_string: String = String::new();
        // loop over input to find sets of same chars
        let mut same_chars: String = String::new();
        for c in input.chars() {
            // If same shars is empty or its the same char in same_chars, push
            // the new char onto the string.
            if same_chars.is_empty() || same_chars.chars().last().unwrap() == c {
                same_chars.push(c);
                //println!("Found: {}", c);
            } else {
                //println!(
                //    "Found new char: {} - Last group was {} long",
                //    c,
                //    same_chars.len()
                //);
                new_string.push_str(same_chars.len().to_string().as_str());
                new_string.push(same_chars.chars().last().unwrap());
                same_chars.clear();
                same_chars.push(c);
            }
        }
        //println!(
        //    "Found new char: {} - Last group was {} long",
        //    same_chars.chars().last().unwrap(),
        //    same_chars.len()
        //);
        new_string.push_str(same_chars.len().to_string().as_str());
        new_string.push(same_chars.chars().last().unwrap());
        same_chars.clear();
        input.clear();
        input.push_str(new_string.as_str());
        println!("[{}] Input is now: {}", i, input);
    }

    println!("After {} iterations, the length is: {}", max_i, input.len());
}
