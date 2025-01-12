use std::env;
use std::process;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

struct House {
    x: i64,
    y: i64,
}

fn main() {
    // Read the arguments to get the filename
    let args: Vec<String> = env::args().collect();
    // Expect 2 arguments <binary> <inputfilename>
    if args.len() < 2 {
        println!("Usage: {} <inputfilename>", &args[0]);
        process::exit(1);
    }
    let filename = Path::new(&args[1]);

    // Open the file for reading
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(_) => {
            println!("Unable to open file!");
            process::exit(2);
        },
    };
    let lines = io::BufReader::new(file).lines();

    let mut santa_house = House {
        x: 0,
        y: 0,
    };
    let mut robot_house = House {
        x: 0,
        y: 0,
    };
    let mut visited: HashMap<String, usize> = HashMap::new();
    visited.insert(
        format!("{},{}", "0", "0"),
        2,
    );

    // Iterate over lines
    for line in lines.map_while(Result::ok) {
        //skip empty lines
        if line.is_empty() {
            continue;
        }

        // Keep track of which santa just went.
        let mut last_santa = "robot";

        // loop over characters in line
        for c in line.trim().chars() {

            // Set the current house to point to santa_house be default
            // change current house based on last_sata. Each loop of char will
            // toggle santa between real and robot.
            let mut current_house: &mut House = &mut santa_house;
            match last_santa {
                "robot" => {
                    last_santa = "real";
                },
                "real" => {
                    current_house = &mut robot_house;
                    last_santa = "robot";
                },
                _ => {
                    println!("Invalid last santa!");
                    process::exit(3);
                },
            }
            
            match c {
                '>' => {
                    current_house.x += 1;
                },
                '<' => {
                    current_house.x -= 1;
                },
                'v' => {
                    current_house.y += 1;
                },
                '^' => {
                    current_house.y -= 1;
                }
                _ => {
                    println!("WTF is {c}???");
                    continue;
                },
            }

            match visited.get(
                &format!("{},{}",
                    current_house.x.to_string(),
                    current_house.y.to_string()
                )
            ) {
                Some(value) => {
                    let newvalue: usize = value + 1;
                    visited.insert(
                        format!("{},{}",
                            current_house.x.to_string(),
                            current_house.y.to_string()
                        ),
                        newvalue
                    );
                },
                None => {
                    visited.insert(
                        format!("{},{}",
                            current_house.x.to_string(),
                            current_house.y.to_string()
                        ),
                        1
                    );
                },
            }
        }
    }

    println!("We visited {} houses!", visited.len());
}
