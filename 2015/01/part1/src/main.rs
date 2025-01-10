use std::env;
use std::process;
use std::fs;

fn main() {
    // Read arguments to get filename
    let args: Vec<String> = env::args().collect();
    // Expect 2 arguments <binary> <inputfilename>
    if args.len() < 2 {
        println!("Usage: {} <inputfilename>", &args[0]);
        process::exit(1);
    }
    let filename = &args[1];
    
    // Open the provided inputfile
    let input = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(_) => {
            println!("There was a problem reading from {}", filename);
            process::exit(2);
        },
    };
    
    let mut floor: isize = 0;

    for c in input.trim().chars() {
        if c == '(' {
            floor += 1;
        } else if c == ')' {
            floor -= 1;
        } else {
            println!("An invalid character was encountered: {c}");
            process::exit(3);
        }
    }

    println!("Santa ended up on floor: {floor}");
}
