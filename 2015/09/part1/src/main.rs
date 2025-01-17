use std::env;
use std::process;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};
use ordermap::{OrderSet, OrderMap};

fn main() {
    // Read in input file
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <inputfilename>", &args[0]);
        process::exit(1);
    }
    let filename = Path::new(&args[1]);
    let file: File = match File::open(&filename) {
        Ok(fp) => fp,
        Err(_) => {
            println!("Unable to open file!");
            process::exit(2);
        }
    };
    let lines = io::BufReader::new(file).lines();

    let mut routes: OrderMap<String, OrderMap<String, usize>> = OrderMap::new();

    println!("Reading in routes!");

    for line in lines.map_while(Result::ok) {
        // skip empty lines
        if line.is_empty() {
            continue;
        }

        // Parse the line
        // <start> to <end> = <cost>
        let line_parts: Vec<&str> = line.trim().split(" = ").collect();
        if line_parts.len() != 2 {
            println!("Unable to parse line!");
            process::exit(3);
        }
        let cost: usize = match line_parts[1].trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Unable to parse cost!");
                process::exit(4);
            }
        };
        let location_parts: Vec<&str> = line_parts[0]
            .trim().split(" to ").collect();
        if location_parts.len() != 2 {
            println!("Unable to parse locations!");
            process::exit(5);
        }
        
        let mut next_routes: OrderMap<String, usize> =
            match routes.get(location_parts[0])
        {
            Some(om) => om.clone(),
            None => OrderMap::new()
        };
        let mut next_routes_inverse: OrderMap<String, usize> =
            match routes.get(location_parts[1])
        {
            Some(om) => om.clone(),
            None => OrderMap::new(),
        };

        next_routes.insert(location_parts[1].to_string(), cost);
        next_routes_inverse.insert(location_parts[0].to_string(), cost);
        routes.insert(location_parts[0].to_string(), next_routes);
        routes.insert(location_parts[1].to_string(), next_routes_inverse);
    }

    //println!("{:#?}", routes);

    let mut found_paths: Vec<OrderSet<String>> = Vec::new();
    let mut found_costs: Vec<usize> = Vec::new();

    for starting_location in routes.keys() {
        //println!("Starting at {}", starting_location);

        let mut visited: OrderSet<String> = OrderSet::new();
        visited.insert(starting_location.to_string());

        dfs_routes(
            starting_location.to_string(),
            &mut visited,
            0,
            &mut found_paths,
            &mut found_costs,
            &routes
        );

    }

    let mut lowest_cost: usize = 0;
    let mut lowest_cost_index: usize = 0;
    for (i, cost) in found_costs.iter().enumerate() {
        if lowest_cost == 0 || *cost < lowest_cost {
            lowest_cost = *cost;
            lowest_cost_index = i;
        }
    }
    
    println!("Cheapest path cost {} - {:?}", lowest_cost, found_paths[lowest_cost_index]);
}

fn dfs_routes(
    location: String,
    visited: &mut OrderSet<String>,
    cost: usize,
    found_paths: &mut Vec<OrderSet<String>>,
    found_costs: &mut Vec<usize>,
    routes: &OrderMap<String, OrderMap<String, usize>>,
) {
    //println!("We're in {}", location);
    let next_routes = routes.get(&location).unwrap();
    for (next_location, next_cost) in next_routes.iter() {
        //println!("From {} we could go to {} for {}", location, next_location, next_cost);
        if !visited.contains(next_location) {
            //println!("Haven't been there yet! Lets go!");
            let mut v2 = visited.clone();
            v2.insert(next_location.to_string());
            let new_cost = cost + next_cost;
            dfs_routes(
                next_location.to_string(),
                &mut v2,
                new_cost,
                found_paths,
                found_costs,
                routes
            );
        } else {
            //println!("But we've already been there");
        }
    }
    // No more children to explore on this path
    // Store it if we've visited all nodes
    if visited.len() == routes.len() {
        //println!("Found path: {:?} for {}", visited, cost);
        found_paths.push(visited.clone());
        found_costs.push(cost);
    }
}
