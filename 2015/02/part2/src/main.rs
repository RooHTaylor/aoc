use std::env;
use std::process;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // Read arguments to get filename
    let args: Vec<String> = env::args().collect();
    // Expect 2 arguments <binary> <inputfilename>
    if args.len() < 2 {
        println!("Usage: {} <inputfilename>", &args[0]);
        process::exit(1);
    }
    let filename = Path::new(&args[1]);
    
    let mut total_ribbon: usize = 0;
    // Open the provided inputfile to read line-by-line
    if let Ok(lines) = read_lines(filename) {
        for line in lines.map_while(Result::ok) {
            // skip empty lines
            if line.is_empty() {
                continue;
            }

            // Split each line on the x char.
            let split_line: Vec<&str> = line.split('x').collect();

            // Confirm we have only 3 elements. l w h
            if split_line.len() != 3 {
                println!("Wrong number of elements found on line!");
                continue;
            }

            // Pull out l w h and convert to number for maths
            let l: usize = match split_line[0].trim().parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("Couldn't convert length! {}", split_line[0]);
                    continue;
                },
            };
            
            let w: usize = match split_line[1].trim().parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("Couldn't convert width! {}", split_line[1]);
                    continue;
                },
            };


            let h: usize = match split_line[2].trim().parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("Couldn't convert height! {}", split_line[2]);
                    continue;
                },
            };

            let perims = vec![2*l + 2*w, 2*w + 2*h, 2*h + 2*l];
            let min_perim = match perims.iter().min() {
                Some(min) => min,
                None => {
                    println!("No perimiters? Impossible!");
                    continue;
                },
            };

            let new_ribbon = l*w*h + min_perim;

            total_ribbon += new_ribbon;
        }
    }

    println!("The total feet of ribbon needed is: {}", total_ribbon);
}

// Function to open a file and return an iterable buffer that can be looped over.
fn read_lines(filename: &Path) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
