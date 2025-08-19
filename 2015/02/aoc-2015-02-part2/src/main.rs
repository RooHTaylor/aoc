use shared::*;

fn main() {
    let args = parse_args();
    
    let file_contents = load_input_file(&args[1]);

    let total_length: usize = calculate_ribbon_length(&file_contents);

    println!("The total length of ribbon required is {} feet", total_length);
}

// Calculate the total length of ribbon needed for all the presents.
// Smallest perimeter of any face, AND cubic feet of volume of the present
fn calculate_ribbon_length(presents: &str) -> usize {
    
    let mut total_length: usize = 0;

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

        let perimeters= [2*length+2*width, 2*width+2*height, 2*height+2*length];
        let smallest_perimeter = match perimeters.iter().min() {
            Some(n) => n,
            None => {
                eprintln!("There was a problem finding the smallest perimeter with {present}");
                continue;
            },
        };

        let present_ribbon_length = length * width * height + smallest_perimeter;

        total_length += present_ribbon_length;
    }

    total_length
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