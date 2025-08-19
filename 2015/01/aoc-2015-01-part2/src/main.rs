use shared::*;

fn main() {
    let args = parse_args();
    
    let file_contents = load_input_file(&args[1]);

    let position = move_santa(&file_contents);

    // If position ends up > the number of instruction in the input-file, we never
    // went into the basement after following all of the directions.
    if position > file_contents.len() {
        println!("Santa never ended up in the basement");
    } else {
        println!("Santa ended up in the basement in position {position}");
    }
}

// Move Santa by following the directions in the file.
// ( = +1
// ) = -1
fn move_santa(directions: &str) -> usize {

    // Santa starts on floor 0
    let mut floor: isize = 0;
    // We start in posiiton 1
    let mut position: usize = 1;
    let directions: Vec<char> = directions.chars().collect();

    for direction in directions {
        match direction {
            '(' => {
                floor += 1;
            },
            ')' => {
                floor -= 1;
            },
            _ => {
                eprintln!("Invalid char {direction} encountered!");
            }
        }
        // Leave as soon as we go into the basement
        if floor < 0 {
            break;
        }
        // If we haven't left the loop, we're going to the next position
        position += 1;
    }

    position
}