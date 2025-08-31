use shared::*;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let args = parse_args();
    let mut debug: bool = false;
    if args.len() == 3 && &args[2] == "--debug" {
        debug = true;
    }
    
    let file_contents = load_input_file(&args[1]);
    if debug { println!("{file_contents}"); }

    let happiness_data: HashMap<String, HashMap<String, isize>> = parse_happiness_data(&debug, &file_contents);

    let (best_happiness, best_arrangement) = calculate_seating(&debug, &happiness_data, isize::min_value(), vec![], vec![]);

    println!("The best change in happiness is: {}\n{}", best_happiness, best_arrangement.join(", "));
}

// Iterate over each possible seating combination to find the MAX change in happieness
fn calculate_seating(debug: &bool, happiness_data: &HashMap<String, HashMap<String, isize>>, best_happiness: isize, best_arrangement: Vec<String>, current_arrangement: Vec<String>) -> (isize, Vec<String>) {
    let mut best_happiness = best_happiness;
    let mut best_arrangement = best_arrangement;

    for (k, _) in happiness_data {
        if current_arrangement.contains(k) {
            if *debug { println!("{} has already been seated", k); }
            continue;
        }
        let mut new_arrangement = current_arrangement.clone();
        new_arrangement.push(k.clone());
        if *debug { println!("Adding {}\n{}", k, new_arrangement.clone().join(", ")); }
        (best_happiness, best_arrangement) = calculate_seating(debug, happiness_data, best_happiness, best_arrangement, new_arrangement.clone());
    }

    if current_arrangement.len() == happiness_data.len() {
        if *debug { println!("Everyone has been seated"); }
        let happiness = calculate_happiness(debug, happiness_data, &current_arrangement);
        if happiness > best_happiness {
            best_happiness = happiness;
            best_arrangement = current_arrangement.clone();
        }
    }

    (best_happiness, best_arrangement)
}

// Calculate the total_happiness change using a current arrangement and the happiness data
//
// Panics!
// This function will panic if a name is in the current arrangement that doesn't
// exist in the happiness_data. This should never happen.
fn calculate_happiness(debug: &bool, happiness_data: &HashMap<String, HashMap<String, isize>>, current_arrangement: &Vec<String>) -> isize {

    if *debug { println!("Calculating happiness for: {}", current_arrangement.clone().join(", ")); }
    let mut total_happiness: isize = 0;
    for (i, _) in current_arrangement.iter().enumerate() {
        // The previous person for the first person is the last person
        let prev_i = if i == 0 {
            current_arrangement.len()-1
        } else {
            i - 1
        };

        // Add previous person to current person
        let previous_person_data = happiness_data.get(&current_arrangement[prev_i]).unwrap();
        total_happiness += previous_person_data.get(&current_arrangement[i].to_string()).unwrap();
        if *debug { println!("{} {} {}", &current_arrangement[prev_i].to_string(), previous_person_data.get(&current_arrangement[i].to_string()).unwrap(), &current_arrangement[i].to_string()); }

        // Add current person to previous person
        let current_person_data = happiness_data.get(&current_arrangement[i]).unwrap();
        total_happiness += current_person_data.get(&current_arrangement[prev_i].to_string()).unwrap();
        if *debug { println!("{} {} {}", &current_arrangement[i].to_string(), current_person_data.get(&current_arrangement[prev_i].to_string()).unwrap(), &current_arrangement[prev_i].to_string()); }
    }

    if *debug { println!("Total happineness change: {}", total_happiness); }
    
    total_happiness
}

// Parse the input using regex
// Should be in the format <person1> would <sign> <amount> happiness units by sitting next to <person2>.
//
// Panics!
// This function will panic if the regex fails to build
fn parse_happiness_data(debug: &bool, input: &str) -> HashMap<String, HashMap<String, isize>> {

    let mut happiness_data: HashMap<String, HashMap<String, isize>> = HashMap::new();

    // Build the regex
    //    <person1> would <sign> <amount> happiness units by sitting next to <person2>.
    let re = Regex::new(
        r"(?P<person1>[A-Za-z]+) would (?P<sign>(lose|gain)) (?P<happiness>[0-9]+) happiness units by sitting next to (?P<person2>[A-Za-z]+)\."
    ).unwrap();

    // Loop over each line and create or update the data for each person
    for line in input.lines() {
        if line.is_empty() { if *debug { println!("Empty line"); } continue; }

        let caps = match re.captures(line) {
            Some(c) => c,
            None => {
                eprintln!("The line did not match the regex! Is there a problem?\n\t{line}");
                continue;
            }
        };
        if *debug { println!("Found: {}, {}, {}, {}",
            caps["person1"].to_string(),
            caps["sign"].to_string(),
            caps["happiness"].to_string(),
            caps["person2"].to_string()); }

        // Add or create the happiness entry for person1 next to person2
        let person1_data = happiness_data
            .entry(caps["person1"]
            .to_string())
            .or_insert_with(HashMap::new);
        let happiness = match caps["sign"].to_string().as_str() {
            "gain" => {
                let num: isize = match caps["happiness"].parse() {
                    Ok(n) => n,
                    Err(_) => {
                        eprintln!("Unable to convert {} to isize!", caps["happiness"].to_string());
                        continue;
                    }
                };
                num
            },
            "lose" => {
                let num: isize = match caps["happiness"].parse() {
                    Ok(n) => n,
                    Err(_) => {
                        eprintln!("Unable to convert {} to isize!", caps["happiness"].to_string());
                        continue;
                    }
                };
                num * -1
            },
            _ => {
                eprintln!("Invalid sign encountered! {}", caps["sign"].to_string());
                continue;
            }
        };
        person1_data.insert(caps["person2"].to_string(),happiness);

    }

    // Add self to each person and create a self entry
    let mut me_data: HashMap<String, isize> = HashMap::new();
    for (k, v) in &mut happiness_data {
        v.insert("Me".to_string(), 0);
        me_data.insert(k.clone(), 0);
    }
    happiness_data.insert("Me".to_string(), me_data);

    if *debug { println!("{:#?}", happiness_data); }
    happiness_data
}