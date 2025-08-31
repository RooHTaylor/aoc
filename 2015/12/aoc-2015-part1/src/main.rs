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

fn iterate_json_for_sum(debug: &bool, input: Value, sum: i64) -> i64 {
    let mut sum = sum;

    if let Some(a) = input.as_array() {
        for v in a {
            sum = sum_or_step(&debug, v.clone(), sum);
        }
    } else if let Some(o) = input.as_object() {
        for (_, v) in o {
            sum = sum_or_step(&debug, v.clone(), sum);
        }
    } else {
        // Do nothing. This value is a string, bool, or null
        eprintln!("An invalid type was encountered.  This should not happen!");
        process::exit(1);
    }

    sum
}

fn sum_or_step(debug: &bool, value: Value, sum: i64) -> i64 {
    let mut sum = sum;

    if value.is_number() {
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

    sum
}