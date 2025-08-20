use shared::*;
use regex::Regex;

fn main() {
    let args = parse_args();
    
    let file_contents = load_input_file(&args[1]);

    let mut grid: Vec<Vec<usize>> = vec![vec![0; 1000]; 1000];
    
    let directions = parse_directions(&file_contents);

    execute(&mut grid, &directions);

    let total_brightness: usize = sum_brightness(&grid);

    println!("The total brightness is {total_brightness}");
}

fn execute(grid: &mut Vec<Vec<usize>>, directions: &Vec<(String, usize, usize, usize, usize)>) {
    for direction in directions {
        for y in direction.2..=direction.4 {
            for x in direction.1..=direction.3 {
                match direction.0.as_str() {
                    "turn on" => {
                        grid[y][x] += 1;
                    },
                    "turn off" => {
                        if grid[y][x] >= 1 {
                            grid[y][x] -= 1;
                        }
                    },
                    "toggle" => {
                        grid[y][x] += 2;
                    },
                    _ => {
                        eprintln!("An unknown action was encountered! {}", direction.0);
                    }
                }
            }
        }
    }
}

// Pull the relevent information out of the direction line.
//
// # Panics!
//
// This function will panic if the regex cannot compile.
// This function will also panic if for some reason the regex matches numerical 
// data that is invalid
fn parse_directions(lines: &str) -> Vec<(String, usize, usize, usize, usize)> {
    let re = Regex::new(
        r"(?P<action>(turn (on|off))|toggle) (?P<sx>[0-9]{1,3}),(?P<sy>[0-9]{1,3}) through (?P<ex>[0-9]{1,3}),(?P<ey>[0-9]{1,3})"
    ).unwrap();

    let mut directions = Vec::new();

    for line in lines.lines() {
        let caps = match re.captures(line.trim()) {
            Some(c) => c,
            None => {
                eprintln!("There was a problem parsing the direction line: {line}");
                continue;
            }
        };

        let action = caps["action"].to_string();
        // Safe to use unwrap here, since the regex should only match digits
        let sx: usize = caps["sx"].parse().unwrap();
        let sy: usize = caps["sy"].parse().unwrap();
        let ex: usize = caps["ex"].parse().unwrap();
        let ey: usize = caps["ey"].parse().unwrap();

        directions.push((action, sx, sy, ex, ey));
    }

    directions
}

fn sum_brightness(grid: &Vec<Vec<usize>>) -> usize {
    let mut sum: usize = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            sum += grid[y][x];
        }
    }

    sum
}

fn _visualize_grid(grid: &Vec<Vec<bool>>) {
    for (y, _) in grid.iter().enumerate() {
        for light in &grid[y] {
            if *light {
                print!("O");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}