use shared::*;
use std::collections::HashMap;

fn main() {
    let args = parse_args();
    
    let file_contents = load_input_file(&args[1]);

    let visits = move_santa(&file_contents);

    println!("Santa visited {} distinct houses!", visits.len());
}

// Follow the ^v<> directions to move santa to a new house.
// Track each house in a HashMap with the key to be the x,y coordinates of the
// house and the value to be the number of times it was visited.
fn move_santa(directions: &str) -> HashMap<String, usize> {

    let mut  x: isize = 0;
    let mut y: isize = 0;

    let mut visits: HashMap<String, usize> = HashMap::new();

    for (i, dir) in directions.trim().chars().enumerate() {
        let mut location_string = String::new();
        location_string.push_str(&x.to_string());
        location_string.push(',');
        location_string.push_str(&y.to_string());

        let num_visits = match visits.get(&location_string) {
            Some(v) => {
                visits.insert(location_string.clone(), v + 1);
                visits.get(&location_string).unwrap()

            },
            None => {
                visits.insert(location_string.clone(), 1);
                // This unwrap should never panic, we just inserted the key
                visits.get(&location_string).unwrap()
            }
        };

        match dir {
            '^' => y -= 1,
            'v' => y += 1,
            '<' => x -= 1,
            '>' => x += 1,
            _ => {
                // If an invalid character is discovered we don't move, so on the
                // next iteration of the loop the current positions visits will 
                // be incremented.  To resolve, decrement here, unless we're on 
                // the last direction
                eprintln!("An invalid direction was encountered: {dir}");
                if i == directions.len() - 1 {
                    eprintln!("Decrementing {x},{y}");
                    visits.insert(location_string, num_visits - 1);
                }
                continue;
            }
        }
    }

    visits
}