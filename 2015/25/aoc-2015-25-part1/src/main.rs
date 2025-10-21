use clap::Parser;
use std::process;

/// Advent Of Code 2015 Day 25 Part 1
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Search row coordinate
    #[arg(short)]
    row: usize,

    /// Search column coordinate
    #[arg(short)]
    column: usize,

    /// Toggle debug messages
    #[arg(long, default_value_t = false)]
    debug: bool,
}

fn main() {
    // Parse args
    let args = Args::parse();

    if args.row < 1 || args.column < 1 {
        eprintln!("The target coordinates are out of scope!");
        process::exit(1);
    }

    let grid = generate_grid(args.debug, args.column, args.row);
    
    let row = args.row - 1;
    let column = args.column - 1;
    if args.debug { println!("{:?}", grid); }
    println!("The code at column {} row {} is {}", args.column, args.row, grid[row][column]);
}

fn generate_grid(
    _debug: bool,
    target_x: usize,
    target_y: usize,
) -> Vec<Vec<usize>> {

    let mut grid: Vec<Vec<usize>> = vec![vec![0; target_x + target_y]; target_y + target_x];

    let mut last_value: usize = 20151125;
    for row in 0..grid.len() {
        last_value = fill_diagonal(_debug, row, last_value, &mut grid);
    }

    grid
}

fn fill_diagonal(
    debug: bool,
    start_y: usize,
    start_value: usize,
    grid: &mut Vec<Vec<usize>>,
) -> usize {
    let mut next_y = start_y;
    let mut next_x: usize = 0;
    let mut next_value = start_value;
    loop {
        grid[next_y][next_x] = next_value;
        if debug { println!("Setting x:{}y:{} to {}", next_x, next_y, next_value); }

        next_value = generate_code(debug, next_value);

        // Try to step up 1 row.  If we overflow this was 0 - break;
        next_y = match next_y.checked_sub(1) {
            Some(n) => n,
            None => {
                break;
            }
        };
        // Try to step over one column
        next_x += 1;
        if next_x > grid[next_y].len() {
            break;
        }
    }

    next_value
}

fn generate_code(_debug: bool, input: usize) -> usize {
    
    let new_code = (input * 252533) % 33554393;

    new_code
}