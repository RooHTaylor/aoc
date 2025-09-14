use shared::*;
use std::{
    process,
    thread,
    time,
};

fn main() {
    let args = parse_args_iterations();
    let mut debug: bool = false;
    if (args.len() == 3 && &args[2] == "--debug") || (args.len() == 4 && (&args[2] == "--debug" || &args[3] == "--debug")) {
        debug = true;
    }

    // Pull the iterations out of the args
    let iterations: usize = match args.len() {
        3 if args[2].parse::<usize>().is_ok() => args[2].parse::<usize>().unwrap(),
        4 if args[2].parse::<usize>().is_ok() || args[3].parse::<usize>().is_ok() => {
            if args[2].parse::<usize>().is_ok() {
                args[2].parse::<usize>().unwrap()
            } else {
                args[3].parse::<usize>().unwrap()
            }
        },
        _ => 100,
    };
    
    let file_contents = load_input_file(&args[1]);
    if debug { println!("{file_contents}"); }

    let mut grid: Vec<Vec<bool>> = parse_grid(&file_contents);

    if debug { println!("{:#?}", grid); }

    print_grid(&debug, &grid);
    thread::sleep(time::Duration::from_millis(500)); // Pause for half a second
    for _ in 0..iterations {
        grid = iterate_grid(&debug, &grid);
        print_grid(&debug, &grid);
        thread::sleep(time::Duration::from_millis(500)); // Pause for half a second
    }

    let lights_on = count_on(&grid);

    println!("There are {lights_on} lights on after {iterations} iterations.");

}

fn count_on(grid: &Vec<Vec<bool>>) -> usize {
    let mut count: usize = 0;
    for line in grid {
        for v in line {
            if *v {
                count += 1;
            }
        }
    }

    count
}

// Display the grid
fn print_grid(debug: &bool, grid: &Vec<Vec<bool>>) {
    // Clear the screen
    if !*debug { print!("\x1B[2J\x1B[1;1H"); }

    for line in grid.iter() {
        for &c in line.iter() {
            print!("{}", if c { '#' } else { '.' });
        }
        print!("\n");
    }

    if *debug { println!(""); }
}

// Perform one iteration of the grid, where each true stays true if 2 or 3 neighbours
// are also true and otherwise goes false, and each false turns true if exactly 
// 3 neighbours are true.
fn iterate_grid(debug: &bool, grid: &Vec<Vec<bool>>) -> Vec<Vec<bool>>{
    let mut new_grid = grid.clone();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if *debug { println!("x:{},y:{}", x, y); }

            // Possible neighbour positions
            let modifiers: Vec<(isize, isize)> = vec![
                (-1,-1),
                (0, -1),
                (1, -1),
                (-1, 0),
                (1, 0),
                (-1, 1),
                (0, 1),
                (1, 1)
            ];
            let mut on_neighbours: usize = 0;
            // For each possible neighbour
            for (xm, ym) in modifiers {
                // Get modified x value, with out of bounds checks
                let mx = match xm {
                    -1 => match x.checked_sub(1) {
                        Some(n) => n,
                        None => {
                            if *debug { println!("{x}-1 is out of bounds!"); }
                            continue;
                        },
                    },
                    0 => x.clone(),
                    1 => match x.checked_add(1) {
                        Some(n) => n,
                        None => {
                            if *debug { println!("{x}+1 is out of bounds!"); }
                            continue;
                        },
                    },
                    _ => {
                        eprintln!("Impossible modifier! {xm}");
                        continue;
                    },
                };
                // Get modified y value, with out of bounds checks
                let my = match ym {
                    -1 => match y.checked_sub(1) {
                        Some(n) => n,
                        None => {
                            if *debug { println!("{y}-1 is out of bounds!"); }
                            continue;
                        },
                    },
                    0 => y.clone(),
                    1 => match y.checked_add(1) {
                        Some(n) => n,
                        None => {
                            if *debug { println!("{y}+1 is out of bounds!"); }
                            continue;
                        },
                    },
                    _ => {
                        eprintln!("Impossible modifier! {ym}");
                        continue;
                    },
                };

                // Check for out of bounds the other way.
                if mx >= grid[y].len() || my >= grid.len() {
                    if *debug { println!("Grid is {} by {}, and values are {} and {}", grid[y].len(), grid.len(), mx, my); }
                    continue;
                }

                // If the neighbour grid position is on, count it
                if grid[my][mx] {
                    on_neighbours += 1;
                    if *debug { println!("Neighbour {mx},{my} is on!"); }
                }
            }
            if *debug { println!("Found {on_neighbours} neighbours on"); }

            // Current is on
            if grid[y][x] {
                // Turn off if on neighbours 2 or 3
                if on_neighbours != 2 && on_neighbours != 3 {
                    if *debug { println!("{x},{y} was on, but does not have 2 or 3 neighbours on"); }
                    new_grid[y][x] = false;
                }
            // Current is off
            } else if !grid[y][x]  {
                // Turn on if on neighbours is 3
                if on_neighbours == 3 {
                    if *debug { println!("{x},{y} was off, and has 3 neighbours on"); }
                    new_grid[y][x] = true;
                }
            }
        }
    }

    new_grid
}

// Create a Vec<Vec<bool>> from a grid file, where . is false and # is true
//
// Exits(1)
// This functon will exit the process if an invalid character is encountered in
// the input file, or if the lines of the input file vary in length.
fn parse_grid(input: &str) -> Vec<Vec<bool>> {
    let mut grid: Vec<Vec<bool>> = Vec::new();
    let mut length: Option<usize> = None;

    for line in input.lines() {
        if line.trim().is_empty() { continue; }

        // Create a Vec<bool> of the line, where . = false and # = true
        let l: Vec<bool> = line.trim().chars().map(|c| match c {
            '.' => false,
            '#' => true,
            _ => {
                eprintln!("ERROR: An invalid character was encountered in the input file!");
                process::exit(1);
            }
        }).collect();

        if length == None {
            length = Some(l.len());
        } else if Some(l.len()) != length {
            eprintln!("ERROR: The input file has lines of different lengths!");
            process::exit(1);
        }

        grid.push(l);
    }

    grid
}