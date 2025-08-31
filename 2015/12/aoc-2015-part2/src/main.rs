use shared::*;
use serde_json::Value;
use std::process;

fn main() {
    let args = parse_args();
    let mut debug: bool = false;
    if args.len() == 3 && &args[2] == "--debug" {
        debug = true;
    }
    
    let file_contents = load_input_file(&args[1]);

    if debug { println!("{file_contents}"); }

    let parsed: Value = match serde_json::from_str(&file_contents) {
        Ok(pj) => pj,
        Err(err) => {
            eprintln!("The provided JSON is invalid and cannot be parsed! {err}");
            process::exit(1);
        }
    };

    if debug { println!("{:#?}", parsed); }

    let mut sum: i64 = 0;

    sum = iterate_json_for_sum(&debug, parsed, sum);

    println!("The sum of all the numbers is: {sum}");
    
}

// Iterate over JSON object or array for a sum
// If an object, start a temporary sum, and return the original sum if the object
// contains property with a value "red"
//
// Exits
// This function will exit the process if the vlaue is not an object or array, as
// it would be impossible and invalid syntax.
fn iterate_json_for_sum(debug: &bool, input: Value, sum: i64) -> i64 {
    let mut sum = sum;

    if let Some(a) = input.as_array() {
        for v in a {
            sum = match sum_or_step(&debug, v.clone(), sum) {
                Some(n) => n,
                None => sum,
            };
        }
    } else if let Some(o) = input.as_object() {
        if *debug { println!("Entering an object.  Starting a temp_sum"); }
        let mut temp_sum: i64 = 0;
        for (_, v) in o {
            temp_sum = match sum_or_step(&debug, v.clone(), temp_sum) {
                Some(n) => n,
                None => {
                    if *debug { println!("\"red\" located! The sum remains unchainged. {sum}"); }
                    return sum;
                }
            };
        }
        if *debug { println!("Made it through the object without finding red.  Adding {} to {}", temp_sum, sum); }
        sum += temp_sum;
    } else {
        // Exit! We should never be iterating over something that isn't array or object
        eprintln!("An invalid type was encountered.  This should not happen!");
        process::exit(1);
    }

    sum
}

// Check the vlaue we have - If it's an object or array, step into it
// If it's a number, convert it to an i64 and add it to the sum.
// If it's red, return None to trigger a bail-out
//
// Exits
// This fuction will exit if the number cannot be converted to i64
fn sum_or_step(debug: &bool, value: Value, sum: i64) -> Option<i64> {
    let mut sum = sum;

    
    if value.is_string() && value.as_str() == Some("red") {
        if *debug { println!("\"red\" located! Bail out!"); }
        return None;

    } else if value.is_number() {
        if *debug { println!("{} is a number!", value); }

        let num = match value.as_i64() {
            Some(n) => n,
            None => {
                eprintln!("Could not convert {} to i64", value);
                process::exit(1);
            }
        };
        
        sum += num;
        
        if *debug { println!("New sum is: {}", sum); }
    } else if value.is_array() || value.is_object() {
        if *debug { println!("The value is an object or array! Stepping in."); }
        sum = iterate_json_for_sum(debug, value.clone(), sum);
    }

    Some(sum)
}