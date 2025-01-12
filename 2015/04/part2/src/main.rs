use std::env;
use std::process;
use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;
use md5;

fn main() {
    // Read the arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <inputfilename>", &args[0]);
        process::exit(1);
    }

    // The input filename
    let filename = Path::new(&args[1]);

    // Open the file and read the lines (buffered)
    let file = match File::open(filename) {
        Ok(fp) => fp,
        Err(_) => {
            println!("Unable to open file!");
            process::exit(2);
        },
    };
    let lines = io::BufReader::new(file).lines();

    let mut input = String::new();
    let mut solution = String::new();

    // Iterate over the lines
    for line in lines.map_while(Result::ok) {
        // Read only one line - should be the puzzle input
        input = line.trim().to_string();
        break;
    }

    for n in 1..=usize::MAX {

        let test_string = format!("{}{}", &input, n.to_string());

        let digest = md5::compute(&test_string);
        let digest = format!("{:#?}", digest);

        match &digest[..6] {
            "000000" => {
                println!("{}", digest);
                solution = n.to_string();
                break;
            },
            _ => (),
        }
    }

    println!("The smallest number is: {}", solution);
}
    
