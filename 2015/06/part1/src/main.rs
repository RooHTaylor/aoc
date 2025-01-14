use std::env;
use std::process;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

fn main() {
    // Read filename from args
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <inputfilename>", args[0]);
        process::exit(1);
    }
    let filename = Path::new(&args[1]);

    // Open file for reading
    let file = match File::open(&filename) {
        Ok(fp) => fp,
        Err(_) => {
            println!("Unable to open file!");
            process::exit(2);
        },
    };
    let lines = io::BufReader::new(file).lines();

    // Initialize grid with all lights off
    let mut grid: Vec<Vec<bool>> = vec![vec![false; 1000]; 1000];
    //print_grid(grid);
    
    let mut total_lights_on: usize = 0;
   
    // Regex for instructions
    // turn on sx,sy through ex,ey
    // turn off sx,sy through ex,ey
    // toggle sx,sy through ex,ey
    let re = Regex::new(
        r"(turn on|turn off|toggle) ([0-9]{1,3}),([0-9]{1,3}) through ([0-9]{1,3}),([0-9]{1,3})"
    ).unwrap();

    // Loop over input lines
    for line in lines.map_while(Result::ok) {

        // skip empty lines
        if line.is_empty() {
            continue;
        }

        // Extract data from regex match of line
        let (
            _, 
            [action, 
            start_x, 
            start_y, 
            end_x, 
            end_y]
        ) = match re.captures(line.trim()).map(|caps| caps.extract()) {
            Some(c) => c,
            None => {
                println!("Unable to process line: {}", line);
                continue;
            }
        };

        //convert sx, sy, ex, ey to usize
        let start_x: usize = match start_x.trim().parse() {
            Ok(sx) => sx,
            Err(_) => {
                println!("Unable to convert start x {} to a number!", start_x);
                continue;
            }
        };
        let start_y: usize = match start_y.trim().parse() {
            Ok(sy) => sy,
            Err(_) => {
                println!("Unable to convert start y {} to a number!", start_y);
                continue;
            }
        };

        let end_x: usize = match end_x.trim().parse() {
            Ok(ex) => ex,
            Err(_) => {
                println!("Unable to convert end x {} to a number!", end_x);
                continue;
            }
        };
        let end_y: usize = match end_y.trim().parse() {
            Ok(ey) => ey,
            Err(_) => {
                println!("Unable to convert end y {} to a number!", end_y);
                continue;
            }
        };

        // Loop through each light in the grid between the start and end
        // which will create a box of lights that we're changing
        //
        // Update the total number of lights on during these changes, to prevent
        // having to loop through all the lights again later.
        for cy in start_y..=end_y {
            for cx in start_x..=end_x {

                // Change the lights in the grid based on the action
                match action {
                    "turn on" => {
                        // if the light is not already on, then there is now a new
                        // light on
                        if !grid[cy][cx] {
                            total_lights_on += 1;
                        }
                        // Turn light on
                        grid[cy][cx] = true;
                    },
                    "turn off" => {
                        // If the light is not already off, then there is one less
                        // light on
                        if grid[cy][cx] {
                            total_lights_on -= 1;
                        }
                        // Turn light off
                        grid[cy][cx] = false;
                    },
                    "toggle" => {
                        // Set the light to whatever the opposide of the light is
                        // now (i.e. toggle it)
                        grid[cy][cx] = !grid[cy][cx];

                        // if the light is now on, then there is a new light on
                        if grid[cy][cx] {
                            total_lights_on += 1;
                        } else {
                            total_lights_on -= 1;
                        }
                    },
                    _ => {
                        println!("Impossible action! {}", action);
                        continue;
                    }
                }
            }
        }
    }

    println!("The total number of lights on is: {}", total_lights_on);
}

// Loop through the grid and print each line with an o if the light is on and and x
// if it is off. This function is for debugging, and need not be used.
fn _print_grid(grid: Vec<Vec<bool>>){
    for y in 0..grid.len() {
        let mut line = String::new();
        for x in 0..grid[y].len() {
            if grid[y][x] {
                line.push('o');
            } else {
                line.push('x');
            }
        }
        println!("{}", line);
    }
}



