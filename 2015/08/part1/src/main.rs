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

    let mut code_length: usize = 0;
    let mut mem_length: usize = 0;

    for line in lines.map_while(Result::ok) {
        // skip empty lines
        if line.is_empty() {
            continue;
        }

        code_length += line.trim().len();
        mem_length += line.trim().len();

        let mut string_length = line.trim().len();

        println!("{line}");
        println!("Code: {}", string_length);


        let mut escaping = false;
        // Loop over each char. If we find an escape character do something
        for c in line.trim().chars() {
            if c == '\\' {
                // If we're already escaping then -1 and stop escaping
                // Otherwise start escaping and move to the next char
                if escaping {
                    //println!("Found escaped \\");
                    mem_length -= 1;
                    string_length -= 1;
                    escaping = false;
                    continue;
                } else {
                    //println!("Found \\ - Escaping!");
                    escaping = true;
                    continue;
                }
            } else if c == '"' {
                // Always remove 1 for " because if " it's the start or end of the
                // string and if \" then it's escaped and is -1 anyways
                //println!("Found \"");
                mem_length -= 1;
                string_length -= 1;
                escaping = false;
                continue;
            } else if c == 'x' && escaping {
                // If we're escaping and the next char is an x then it's an ASCII
                // char and we subtract 3
                //println!("Found an ASCII char");
                mem_length -= 3;
                string_length -= 3;
                escaping = false;
                continue;
            } else {
                // Escaping should never overflow past the second char.
                escaping = false;
                continue;
            }
        }
        println!("String: {}", string_length);
    }

    println!("Code length: {}", code_length);
    println!("Memory length: {}", mem_length);

    let answer = code_length - mem_length;
    println!("Answer: {}", answer);
}
