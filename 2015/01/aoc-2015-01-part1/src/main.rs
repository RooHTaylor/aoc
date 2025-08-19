use shared::*;

fn main() {
    let args = parse_args();
    
    let file_contents = load_input_file(&args[1]);

    // Santa starts at floor 0
    let floor = move_santa(&file_contents);

    println!("Santa ended up on floor {floor}");
}

// Move Santa by following the directions in the file.
// ( = +1
// ) = -1
fn move_santa(directions: &str) -> isize {

    // Santa starts on floor 0
    let mut floor = 0;
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
    }

    floor
}