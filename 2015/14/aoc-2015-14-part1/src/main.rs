use shared::*;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let args = parse_args_iterations();
    let mut debug: bool = false;
    if (args.len() == 3 && &args[2] == "--debug") || (args.len() == 4 && (&args[2] == "--debug" || &args[3] == "--debug")) {
        debug = true;
    }

    let iterations: usize = match args.len() {
        3 if args[2].parse::<usize>().is_ok() => args[2].parse::<usize>().unwrap(),
        4 if args[2].parse::<usize>().is_ok() || args[3].parse::<usize>().is_ok() => {
            if args[2].parse::<usize>().is_ok() {
                args[2].parse::<usize>().unwrap()
            } else {
                args[3].parse::<usize>().unwrap()
            }
        },
        _ => 1,
    };
    
    let file_contents = load_input_file(&args[1]);
    if debug { println!("{file_contents}"); }

    let reindeer_data = parse_reindeer_data(&file_contents);

    let results = run(&debug, &reindeer_data, iterations);

    let mut winner: usize = 0;
    let mut winner_name: String = String::new();
    for (name, distance) in &results {
        if *distance > winner {
            winner = distance.clone();
            winner_name = name.clone();
        }
    }

    if debug { println!("{:#?}", results); }

    println!("{winner_name} won, travelling {winner}km in {iterations} seconds.");
    
}

fn run(debug: &bool, reindeer_data: &HashMap<String, ((usize, usize), usize)>, seconds: usize) -> HashMap<String, usize> {
    // Keep track of the position of each reindeer
    let mut state: HashMap<String, usize> = HashMap::new();
    // Keep track of the countdown of each reindeer
    let mut countdown: HashMap<String, (usize, usize)> = HashMap::new();

    for i in 1..=seconds {
        if *debug { println!("Second {i}"); }

        for (name, ((speed, s_duration), r_duration)) in reindeer_data {
            
            let rstate = state.entry(name.clone()).or_insert(0);
            let (rsduration, rrduration) = countdown.entry(name.clone()).or_insert((s_duration.clone(), r_duration.clone()));

            if *rsduration > 0 {
                if *debug { println!("{name} moving for {rsduration} more seconds"); }
                *rstate += *speed;
                *rsduration -= 1;
                if *rsduration == 0 {
                    if *debug { println!("{name} done moving"); }
                    *rrduration = *r_duration;
                }
            } else {
                if *rrduration > 0 {
                    if *debug { println!("{name} resting for {rrduration} more seconds"); }
                    *rrduration -= 1;
                    if *rrduration == 0 {
                        if *debug { println!("{name} done resting"); }
                        *rsduration = *s_duration;
                    }
                }
            }
        }
    }

    state
}

// Parse the input using regex.
//
// Panics!
// This function will panic if the regex cannot be compiled.
fn parse_reindeer_data(input: &str) -> HashMap<String, ((usize, usize), usize)> {

    let re = Regex::new(
        r"(?P<name>[A-z]+) can fly (?P<speed>[0-9]+) km\/s for (?P<s_duration>[0-9]+) seconds, but then must rest for (?P<r_duration>[0-9]+) seconds\.").unwrap();
    
    let mut reindeer_data: HashMap<String, ((usize, usize), usize)> = HashMap::new();

    for line in input.lines() {
        if line.is_empty() { continue; }

        let caps = match re.captures(line) {
            Some(c) => c,
            None => {
                eprintln!("The line did not match the pattern. {line}");
                continue;
            }
        };
        let speed = match caps["speed"].parse::<usize>() {
            Ok(n) => n,
            Err(err) => {
                eprintln!("Unable to convert speed to usize! {}", err);
                continue;
            }
        };
        let s_duration = match caps["s_duration"].parse::<usize>() {
            Ok(n) => n,
            Err(err) => {
                eprintln!("Unable to convert speed duration to usize! {}", err);
                continue;
            }
        };
        let r_duration = match caps["r_duration"].parse::<usize>() {
            Ok(n) => n,
            Err(err) => {
                eprintln!("Unable to convert rest duration to usize! {}", err);
                continue;
            }
        };

        reindeer_data.insert(caps["name"].to_string(), ((speed, s_duration), r_duration));
    }

    reindeer_data
}