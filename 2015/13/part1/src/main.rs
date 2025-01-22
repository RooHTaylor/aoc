use std::env;
use std::process;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;
use regex::Regex;

fn main() {
    // Read filename from args
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <inputfilename>", &args[0]);
        process::exit(1);
    }
    let filename = Path::new(&args[1]);
    // Open file and read lines into a buffered reader
    let file = match File::open(&filename) {
        Ok(fp) => fp,
        Err(_) => {
            println!("Unable to open file! {}", filename.to_str().unwrap());
            process::exit(2);
        },
    };
    let lines = io::BufReader::new(file).lines();

    // happiness_chart[<person1>][<person2>] = <change in happiness>
    let mut happiness_chart: HashMap<String, HashMap<String, i64>> = 
        HashMap::new();

    // Line regex
    // <person1> would <+/-> <units> happiness units by sitting next to <person2>.
    let line_re = Regex::new(
        r"^(?<person1>\w+) would (?<sign>lose|gain) (?<amount>\d+) happiness units by sitting next to (?<person2>\w+)\.$"
    ).unwrap();

    for line in lines.map_while(Result::ok) {
        // skip empty lines
        if line.is_empty() {
            continue;
        }
        // Extract the regex captures from the line. If extraction fails, then
        // skip this line, but print an error.
        let Some(parts) = line_re.captures(&line) else {
            println!("Line did not match pattern!");
            continue;
        };
        
        // We should have exactly 5 captures (includes the full line at [0])
        // <full_match> <person1> <sign> <amount> <person2>
        if parts.len() != 5 {
            println!("Got the wrong number of captures!");
            continue;
        }
        
        // if the preferences of the current person do not exist, create the new
        // HashMap and return it.
        let mut preferences: HashMap<String, i64> = 
            match happiness_chart.get(&parts["person1"])
        {
            Some(p) => p.clone(),
            None => {
                let new_people: HashMap<String, i64> = HashMap::new();
                happiness_chart.insert(String::from(&parts["person1"]), new_people);
                happiness_chart.get(&parts["person1"]).unwrap().clone()
            }
        };

        // If the preference of person1 to person2 is not yet in the HashMap then
        // add it, otherwise do nothing (should not encounter preference for the
        // same person twice)
        match preferences.contains_key(&parts["person2"]) {
            false => {
                let mut amount: i64 = match parts["amount"].trim().parse() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Could not parse number! {}", &parts["amount"]);
                        process::exit(5);
                    }
                };
                // We either gain or lose the amount. If its a loss then make it
                // negative.
                if parts["sign"].to_owned() == "lose" {
                    amount = amount * -1;
                }
                preferences.insert(String::from(&parts["person2"]), amount);
            },
            _ => (),
        }

        happiness_chart.insert(String::from(&parts["person1"]), preferences);
    }

    // Generate all the possible arrangements of people
    let mut possible_arrangements: Vec<Vec<String>> = Vec::new();
    seat_people(&mut Vec::new(), &mut possible_arrangements, &happiness_chart);

    // Start with the worst possible score as best score.
    // Loop over each possible arrangement and calculate the score; overwriting
    // the best score for each better one.
    let mut best_score: i64 = i64::MIN;
    let mut possible_scores: Vec<i64> = Vec::new();
    for arrangement in possible_arrangements {
        let mut score: i64 = 0;
        // For each arrangement, we need to look at the change in score for the
        // person to the left of person i and the person to the right.
        // Special cases for position 0 and position .len()-1 to make the vec
        // wrap around.
        // The total delta of each position is the score.
        for i in 0..arrangement.len() {
            let delta_left: &i64;
            let delta_right: &i64;
            if i == 0 {
                delta_left = happiness_chart.get(&arrangement[i]).unwrap()
                    .get(&arrangement[arrangement.len()-1]).unwrap();
                delta_right = happiness_chart.get(&arrangement[i]).unwrap()
                    .get(&arrangement[i+1]).unwrap();
            } else if i == arrangement.len() - 1 {
                delta_left = happiness_chart.get(&arrangement[i]).unwrap()
                    .get(&arrangement[i-1]).unwrap();
                delta_right = happiness_chart.get(&arrangement[i]).unwrap()
                    .get(&arrangement[0]).unwrap();
            } else {
                delta_left = happiness_chart.get(&arrangement[i]).unwrap()
                    .get(&arrangement[i-1]).unwrap();
                delta_right = happiness_chart.get(&arrangement[i]).unwrap()
                    .get(&arrangement[i+1]).unwrap();
            }

            score = score + delta_left + delta_right;
        }

        println!("{:?} - {}", arrangement, score);
        possible_scores.push(score.clone());
        if score > best_score {
            best_score = score;
        }
    }

    println!("Best score is: {}", best_score);
}

fn seat_people(
    seat_order: &mut Vec<String>,
    possible_arrangements: &mut Vec<Vec<String>>,
    happiness_chart: &HashMap<String, HashMap<String, i64>>
) {
    'people: for (person1, _preferences) in happiness_chart.iter() {
        for seat in &mut *seat_order {
            if seat == person1 {
                continue 'people;
            }
        }
        seat_order.push(String::from(person1));

        seat_people(seat_order, possible_arrangements, happiness_chart);
    }

    if seat_order.len() == happiness_chart.len() {
        possible_arrangements.push(seat_order.clone());
    }
    if seat_order.len() > 0 {
        seat_order.remove(seat_order.len()-1);
    }
}
