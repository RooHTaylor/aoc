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

    for line in lines.map_while(Result::ok) {
        println!("{}", line);
    }
}
