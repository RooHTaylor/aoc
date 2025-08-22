use shared::*;
use std::collections::HashMap;

fn main() {
    let args = parse_args();
    let mut debug: bool = false;
    if args.len() == 3 && &args[2] == "--debug" {
        debug = true;
    }
    
    let file_contents = load_input_file(&args[1]);

    let all_routes: HashMap<String, HashMap<String, usize>> = parse_locations(debug, &file_contents);

    if debug { println!("{:#?}", all_routes); }

    let (route, cost) = find_longest_route(debug, &all_routes);

    println!("Longest route: {} = {cost}", route.join(" -> "));
}

fn find_longest_route(debug: bool, all_routes: &HashMap<String, HashMap<String, usize>>) -> (Vec<String>, usize) {

    let mut longest_route: Vec<String> = Vec::new();
    let mut longest_route_cost: usize = 0;
    // Start at each possible start location and traverse
    for (start, _) in all_routes {
        let route: Vec<String> = vec![start.clone()];
        if debug { println!("Starting at {start}"); }
        if debug { println!("Route: {}", route.join(" -> ")); }
        (longest_route, longest_route_cost) = traverse_route(debug, &all_routes, start, route.clone(), 0, longest_route.clone(), longest_route_cost);
    }

    (longest_route, longest_route_cost)
}

fn traverse_route(
    debug: bool,
    all_routes: &HashMap<String, HashMap<String, usize>>,
    location: &String,
    route: Vec<String>,
    cost: usize,
    mut longest_route: Vec<String>,
    mut longest_route_cost: usize
 ) -> (Vec<String>, usize) {

    let next_routes = match all_routes.get(location) {
        Some(r) => r,
        None => {
            eprintln!("We got an invalid location!");
            return (longest_route, longest_route_cost);
        }
    };

    for (destination, c) in next_routes {
        // Have we already been here?
        if route.contains(destination) {
            continue;
        }
        let mut new_route = route.clone();
        new_route.push(destination.clone());
        let new_cost = cost + c;
        if debug { println!("Traveling to {destination} with cost {c}"); }
        if debug { println!("Route: {}", new_route.join(" -> ")); }
        if debug { println!("Cost: {new_cost}"); }
        (longest_route, longest_route_cost) = traverse_route(debug, all_routes, destination, new_route.clone(), new_cost, longest_route.clone(), longest_route_cost);
        if debug { println!("Back to route: {}", route.join(" -> ")); }
    }

    // We've been everywhere!
    if route.len() == all_routes.len() {
        if debug { println!("We've been everywhere!"); }
        if debug { println!("Route: {}", route.join(" -> ")); }
        if debug { println!("Cost: {cost}"); }
        if debug { println!("Current Sortest Route: {}", longest_route.join(" -> ")); }
        if debug { println!("Current Cost: {longest_route_cost}"); }
        if cost > longest_route_cost {
            if debug { println!("THIS IS THE NEW LONGEST ROUTE!"); }
            longest_route = route.clone();
            longest_route_cost = cost;
        }
    }

    (longest_route, longest_route_cost)
}

fn parse_locations(debug: bool, list: &str) -> HashMap<String, HashMap<String, usize>> {
    let mut all_routes: HashMap<String, HashMap<String, usize>> = HashMap::new();

    for line in list.lines() {
        let parts: Vec<&str> = line.trim().split(" to ").collect();
        if parts.len() != 2 {
            eprintln!("There was a problem parsing a line: {line}");
            continue;
        }
        let from: String = String::from(parts[0]);

        let parts: Vec<&str> = parts[1].split(" = ").collect();
        if parts.len() != 2 {
            eprintln!("There was a problem parsing a line: {line}");
            continue;
        }
        let to: String = String::from(parts[0]);
        let cost: usize = match parts[1].parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("There was a problem parsing a line: {line}");
                continue;
            }
        };

        if debug { println!("Route from {from} to {to} with cost {cost}"); }

        let routes1: &mut HashMap<String, usize> = match all_routes.get_mut(&from.to_string()) {
            Some(r) => {
                if debug { println!("Adding route to {} from {}", to, from); }
                r
            },
            None => {
                if debug { println!("Creating {} Adding route to {} from {}", from, to, from); }
                let destinations: HashMap<String, usize> = HashMap::new();
                all_routes.insert(from.clone(), destinations);
                all_routes.get_mut(&from.to_string()).unwrap()
            },
        };
        routes1.insert(to.to_string(), cost);

        let routes2: &mut HashMap<String, usize> = match all_routes.get_mut(&to.to_string()) {
            Some(r) => {
                if debug { println!("Adding route to {} from {}", from, to); }
                r
            },
            None => {
                if debug { println!("Creating {} Adding route to {} from {}", to, from, to); }
                let destinations: HashMap<String, usize> = HashMap::new();
                all_routes.insert(to.clone(), destinations);
                all_routes.get_mut(&to.to_string()).unwrap()
            },
        };
        routes2.insert(from.to_string(), cost);

    }

    all_routes
}