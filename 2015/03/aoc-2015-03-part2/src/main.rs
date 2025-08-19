use shared::*;
use std::collections::HashMap;

fn main() {
    let args = parse_args();
    
    let file_contents = load_input_file(&args[1]);

    let visits = move_santas(&file_contents);

    println!("The Santas visited {} distinct houses!", visits.len());
}

// Follow the ^v<> directions to move Santa to a new house.
// Track each house in a HashMap with the key to be the x,y coordinates of the
// house and the value to be the number of times it was visited.
fn move_santas(directions: &str) -> HashMap<String, usize> {

    // Both santas start at 0,0
    let mut santa_coords: Vec<(isize, isize)> = vec![(0, 0), (0, 0)];

    // Pre-load the HashMap with 0,0 and a value of 2, sine both santas visit the
    // starting location
    let mut visits: HashMap<String, usize> = HashMap::new();
    visits.insert("0,0".to_string(), 2);

    for (i, dir) in directions.trim().chars().enumerate() {
        // We're going to move Santa 0 for even iterations and Santa 1 for odd 
        // iterations.  Pull the current coordinates for the appropriate Santa
        let santa_to_move = i % 2;
        let (mut x, mut y) = santa_coords[santa_to_move];

        // Update the coordinates according to the direction.
        match dir {
            '^' => y -= 1,
            'v' => y += 1,
            '<' => x -= 1,
            '>' => x += 1,
            _ => {
                eprintln!("An invalid direction was encountered: {dir}");
                continue;
            }
        }

        // Stringify the coordinates
        let mut location_string = String::new();
        location_string.push_str(&x.to_string());
        location_string.push(',');
        location_string.push_str(&y.to_string());

        // Update the visits HashMap - We don't actually need the total number of 
        // visits.
        let _num_visits = match visits.get(&location_string) {
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

        // Store the new coordinates
        santa_coords[santa_to_move] = (x, y);
    }

    visits
}