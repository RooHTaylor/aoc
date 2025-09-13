use shared::*;

fn main() {
    let args = parse_args_iterations();
    let mut debug: bool = false;
    if (args.len() == 3 && &args[2] == "--debug") || (args.len() == 4 && (&args[2] == "--debug" || &args[3] == "--debug")) {
        debug = true;
    }

    // Pull the max out of the args
    let max: usize = match args.len() {
        3 if args[2].parse::<usize>().is_ok() => args[2].parse::<usize>().unwrap(),
        4 if args[2].parse::<usize>().is_ok() || args[3].parse::<usize>().is_ok() => {
            if args[2].parse::<usize>().is_ok() {
                args[2].parse::<usize>().unwrap()
            } else {
                args[3].parse::<usize>().unwrap()
            }
        },
        _ => 150,
    };
    
    let file_contents = load_input_file(&args[1]);
    if debug { println!("{file_contents}"); }

    let buckets = load_bucket_data(&file_contents);

    let mut combinations: Vec<Vec<usize>> = Vec::new();

    recurse_buckets(&debug, &buckets, &mut combinations, &max, Vec::new());

    if debug { println!("{:#?}", combinations); }
    println!("There are {} combinations that make {max}", combinations.len());
}

fn recurse_buckets(debug: &bool, buckets: &Vec<usize>, combinations: &mut Vec<Vec<usize>>, max: &usize, current_combination: Vec<usize>) {
    // Use the index of each bucket, since there can be duplicate bucket values
    for (i, v) in buckets.iter().enumerate() {
        // If we already have this index in the sequence then skip it
        if current_combination.contains(&i) { continue; }

        // Calculate our current bucket total
        let mut sum: usize = 0;
        for bi in current_combination.iter() {
            sum += buckets[*bi];
        }
        if *debug { println!("Current sum: {sum}"); }

        // Skip if we're already over the max or if this value puts us over the max
        if sum >= *max || (sum + *v) > *max {
            if *debug { println!("Already over {max}, or {v} puts us over."); }
            continue;
        // If this value puts at at exactly the max, we can save this combination
        // Sort the combination first, so that we can de-duplicate and only add it once.
        } else if (sum + *v) == *max {
            let mut new_combination = current_combination.clone();
            new_combination.push(i.clone());
            new_combination.sort();
            
            if combinations.contains(&new_combination) {
                if *debug { println!("This combination already exists, skipping."); }
                continue;
            }
            if *debug { println!("Adding {v} hits {max}, saving combination: [{}]", new_combination.iter().map(|num| buckets[*num].to_string()).collect::<Vec<String>>().join(", ")); }
            combinations.push(new_combination.clone());
            continue;
        }

        let mut new_combination = current_combination.clone();
        new_combination.push(i.clone());

        recurse_buckets(debug, buckets, combinations, max, new_combination);
    }
}

// Load the bucket data as a Vec<usize>
fn load_bucket_data(input: &str) -> Vec<usize> {
    let mut buckets: Vec<usize> = Vec::new();

    for line in input.trim().lines() {
        if line.trim().is_empty() { continue; }

        let bucket: usize = match line.trim().parse::<usize>() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Could not convert {} to usize!", line.trim());
                continue;
            }
        };

        buckets.push(bucket);
    }

    buckets
}