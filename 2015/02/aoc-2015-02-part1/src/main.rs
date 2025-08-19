use shared::*;

fn main() {
    let args = parse_args();
    
    let file_contents = load_input_file(&args[1]);

    let total_area = calculate_square_footage(&file_contents);

    println!("The total square footage required is {}", total_area);
}

// Calculate the total square footage of wrapping paper needed to wrap all the
// presents in the list. List should be lxwxh format.
// Formula for calculating the square footage is:
// 2*l*w + 2*w*h + 2*h*l + smallest_side
fn calculate_square_footage(presents: &str) -> usize {
    
    let mut total_area: usize = 0;

    // Iterate over the lines
    let presents: Vec<&str> = presents.lines().collect();
    for present in presents {
        // Skip empty lines
        if present.is_empty() {
            continue;
        }

        let (length, width, height) = match parse_dimensions(&present) {
            Ok((l, w, h)) => (l, w, h),
            Err(msg) => {
                eprintln!("There was a problem with {present}: {msg}");
                continue;
            },
        };

        let sides= [length*width, width*height, height*length];
        let smallest_side = match sides.iter().min() {
            Some(n) => n,
            None => {
                eprintln!("There was a problem finding the smallest side with {present}");
                continue;
            },
        };

        let present_square_footage = 2*length*width + 2*width*height + 2*height*length + smallest_side;

        total_area += present_square_footage;
    }

    total_area
}

// Parse the length width and height from a string formatted lxwxh
// Return a Result, with Ok(l, w, h) or Err(&str)
fn parse_dimensions(line: &str) -> Result<(usize, usize, usize), &str> {

    let parts: Vec<&str> = line.splitn(3, 'x').collect();
    if parts.len() != 3 {
        return Err("We did not find 3 dimensions")
    }

    let length: usize = match parts[0].parse() {
        Ok(n) => n,
        Err(_) => {
            return Err("Length was not a valid number!")
        }
    };

    let width: usize = match parts[1].parse() {
        Ok(n) => n,
        Err(_) => {
            return Err("Width was not a valid number!")
        }
    };

    let height: usize = match parts[2].parse() {
        Ok(n) => n,
        Err(_) => {
            return Err("Height was not a valid number!")
        }
    };

    Ok((length, width, height))
}