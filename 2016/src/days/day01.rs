use aoc_common::Options;
use std::{
    fmt,
    collections::HashSet
};


enum Direction {
    North,
    East,
    South,
    West,
}
impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::North => write!(f, "North"),
            Direction::East => write!(f, "East"),
            Direction::South => write!(f, "South"),
            Direction::West => write!(f, "West"),
        }
    }
}


struct Grid {
    current_direction: Direction,
    x: i64,
    y: i64,
}
impl Grid {
    /// Change the current direction based on a L or R turn.
    fn turn(&mut self, direction: char) -> Result<bool, String>{
        match direction {
            'R' => {
                self.current_direction = match self.current_direction {
                    Direction::North => Direction::East,
                    Direction::East => Direction::South,
                    Direction::South => Direction::West,
                    Direction::West => Direction::North
                };
            }
            'L' => {
                self.current_direction = match self.current_direction {
                    Direction::North => Direction::West,
                    Direction::East => Direction::North,
                    Direction::South => Direction::East,
                    Direction::West => Direction::South
                };
            }
            _ => {
                return Err(format!("An invalid direction was encountered! {}", direction))
            }
        }

        Ok(true)
    }

    /// Update the current grid position based on walking distance units in
    /// current_direction
    fn walk(&mut self, distance: i64) -> Result<bool, String> {
        match self.current_direction {
            Direction::North => {
                self.y += distance;
            },
            Direction::East => {
                self.x += distance;
            },
            Direction::South => {
                self.y -= distance;
            },
            Direction::West => {
                self.x -= distance;
            }
        }

        Ok(true)
    }
}


/// Parse the list of instructions as a L14, R5 format, and return a Vec of the 
/// directions and the number of spaces to move.
fn parse_instructions(input: &str) -> Result<Vec<(char, i64)>, String> {
    // Instructions should be on one line, separated by commas
    let instructions = match input
    .trim()
    .split(',')
    .map(|s| {
        let mut c = s.trim().chars();
        let dir = match c.next() {
            Some('R') => 'R',
            Some('L') => 'L',
            Some(_) | None => {
                return Err(format!("Invalid direction! -{}-", s))
            }
        };
        let Ok(num) = c.as_str().parse::<i64>() else {
            return Err(format!("Invalid number! -{}-", s))
        };

        Ok((dir, num))
    })
    .collect() {
        Ok(v) => v,
        Err(e) => { return Err(e) }
    };

    Ok(instructions)
}


/// Starting at position 0,0 facing north, follow L14, R4 formatted directions
/// to move your position, and then return your distance from the starting 
/// position as the solution.
pub fn part1(input: &str, opts: & Options) -> Result<String, String> {

    let instructions = match parse_instructions(input) {
        Ok(v) => v,
        Err(e) => {
            return Err(format!("Unable to parse instructions: {}", e))
        }
    };

    // We start at 0,0 facing North
    let mut grid = Grid{
        x: 0,
        y: 0,
        current_direction: Direction::North
    };
    if opts.debug {
        eprintln!("Starting at {},{} facing {}", grid.x, grid.y, grid.current_direction);
    }

    for instruction in instructions {

        let _ = grid.turn(instruction.0)?;
        let _ = grid.walk(instruction.1)?;

        if opts.debug {
            eprintln!(
                "Moving {} by {} blocks to arrive at {},{}",
                grid.current_direction,
                instruction.1,
                grid.x,
                grid.y
            );
        }
    }

    let distance = (grid.x.abs() + grid.y.abs()).to_string();
    
    Ok(distance)
}


/// Starting at position 0,0 facing north, follow L14, R4 formatted directions
/// to move your position. Keep track of all the positions you visit, and return
/// the distance from the starting point of the first position you visit twice.
pub fn part2(input: &str, opts: & Options) -> Result<String, String> {

    let instructions = match parse_instructions(input) {
        Ok(v) => v,
        Err(e) => {
            return Err(format!("Unable to parse instructions: {}", e))
        }
    };

    // We start at 0,0 facing North
    let mut grid = Grid{
        x: 0,
        y: 0,
        current_direction: Direction::North
    };
    // Use a HashSet to track history. All we need to do is check if we've 
    // visited this index already.
    let mut history: HashSet<(i64,i64)> = HashSet::new();
    history.insert((0,0));

    if opts.debug {
        eprintln!("Starting at {},{} facing {}", grid.x, grid.y, grid.current_direction);
    }

    'inloop: for instruction in instructions {

        let _ = grid.turn(instruction.0)?;
        for num in 1..=instruction.1 {
            let mut tx = grid.x;
            let mut ty = grid.y;
            match grid.current_direction {
                Direction::North => {
                    ty += num;
                },
                Direction::East => {
                    tx += num;
                },
                Direction::South => {
                    ty -= num;
                },
                Direction::West => {
                    tx -= num;
                }
            }
            // If the coordinates already exist in the HashSet then stop processing
            // We have the first duplicate
            if !history.insert((tx, ty)) {
                if opts.debug {
                    eprintln!("We have visited {},{} before", tx, ty);
                }
                grid.x = tx;
                grid.y = ty;
                break 'inloop;
            }
        }
        let _ = grid.walk(instruction.1)?;
        if opts.debug {
            eprintln!(
                "Moving {} by {} blocks to arrive at {},{}",
                grid.current_direction,
                instruction.1,
                grid.x,
                grid.y
            );
        }
    }

    let distance = (grid.x.abs() + grid.y.abs()).to_string();
    
    Ok(distance)
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE101: &str = "R2, L3";
    const EXAMPLE102: &str = "R2, R2, R2";
    const EXAMPLE103: &str = "R5, L5, R5, R3";
    const EXAMPLE201: &str = "R8, R4, R4, R8";

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(EXAMPLE101, &Options{ debug: false, iterations: None }).unwrap(),
            "5"
        );
        assert_eq!(
            part1(EXAMPLE102, &Options{ debug: false, iterations: None }).unwrap(),
            "2"
        );
        assert_eq!(
            part1(EXAMPLE103, &Options{ debug: false, iterations: None }).unwrap(),
            "12"
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(EXAMPLE201, &Options{ debug: false, iterations: None }).unwrap(),
            "4"
        );
    }
}