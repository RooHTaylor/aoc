use shared::*;
use regex::Regex;

// Load the ingedient list
// Parse ingredients with regex to split into a Vec<Ingredient>
fn main() {
    let args = parse_args();
    let mut debug: bool = false;
    if args.len() == 3 && &args[2] == "--debug" {
        debug = true;
    }
    
    let file_contents = load_input_file(&args[1]);
    if debug { println!("{file_contents}"); }

    let ingredient_list = parse_ingredients(&debug, &file_contents);

    if debug { println!("{:#?}", ingredient_list); }

    let mut max_score: usize = 0;
    permute_ingredient_amounts(&debug, &ingredient_list, &mut vec![0; ingredient_list.len()], 0, 100, &mut max_score);

    println!("The max score is {max_score}");
}

// Permute the amount of each ingredient to find the max
fn permute_ingredient_amounts(
    debug: &bool,
    ingredient_list: &Vec<Ingredient>,
    amounts: &mut Vec<usize>,
    index: usize,
    remaining: usize,
    max_score: &mut usize,
) {
    if index == ingredient_list.len() - 1 {
        amounts[index] = remaining;
        let (score, calories) = calculate_score(&ingredient_list, &amounts);
        if calories == 500 {
            *max_score = (*max_score).max(score);
        }
        return;
    }

    for i in 0..=remaining {
        amounts[index] = i;
        if *debug { println!("{:#?}", amounts); }
        permute_ingredient_amounts(debug, ingredient_list, amounts, index + 1, remaining - i, max_score);
    }
}

// Calculate the score
fn calculate_score(ingredient_list: &Vec<Ingredient>, amounts: &Vec<usize>) -> (usize, usize) {
    let mut capacity = 0;
    let mut durability = 0;
    let mut flavor = 0;
    let mut texture = 0;
    let mut total_calories: usize = 0;

    for (i, ingredient) in ingredient_list.iter().enumerate() {
        let amount = amounts[i] as isize;
        capacity += amount * ingredient.capacity;
        durability += amount * ingredient.durability;
        flavor += amount * ingredient.flavor;
        texture += amount * ingredient.texture;
        let amount = amounts[i] as usize;
        total_calories += amount * ingredient.calories;
    }

    // Ensure no negative totals
    capacity = capacity.max(0);
    durability = durability.max(0);
    flavor = flavor.max(0);
    texture = texture.max(0);

    // Calculate the total score
    let score = (capacity * durability * flavor * texture).try_into().unwrap_or(0);

    (score, total_calories)
}

// Parse each ingredient with regex and build a vec of the ingredients
//
// Panics!
// This function will panic if the regex fails to compile
fn parse_ingredients(debug: &bool, input: &str) -> Vec<Ingredient> {

    let mut ingredient_list: Vec<Ingredient> = Vec::new();

    let re = Regex::new(
        r"(?P<name>[A-z]+): capacity (?P<capacity>\-?[0-9]+), durability (?P<durability>\-?[0-9]+), flavor (?P<flavor>\-?[0-9]+), texture (?P<texture>\-?[0-9]+), calories (?P<calories>\-?[0-9]+)"
    ).unwrap();

    for line in input.lines() {
        if line.is_empty() { continue; }

        let caps = match re.captures(line.trim()) {
            Some(o) => o,
            None => {
                eprintln!("Unable to parse the line: {line}");
                continue;
            }
        };

        let name: String = caps["name"].to_string();
        let capacity: isize = if caps["capacity"].parse::<isize>().is_ok() {
            caps["capacity"].parse().unwrap()
        } else {
            eprintln!("Unable to parse the capacity: {line}");
            continue;
        };
        let durability: isize = if caps["durability"].parse::<isize>().is_ok() {
            caps["durability"].parse().unwrap()
        } else {
            eprintln!("Unable to parse the durability: {line}");
            continue;
        };
        let flavor: isize = if caps["flavor"].parse::<isize>().is_ok() {
            caps["flavor"].parse().unwrap()
        } else {
            eprintln!("Unable to parse the flavor: {line}");
            continue;
        };
        let texture: isize = if caps["texture"].parse::<isize>().is_ok() {
            caps["texture"].parse().unwrap()
        } else {
            eprintln!("Unable to parse the texture: {line}");
            continue;
        };
        let calories: usize = if caps["calories"].parse::<usize>().is_ok() {
            caps["calories"].parse().unwrap()
        } else {
            eprintln!("Unable to parse the calories: {line}");
            continue;
        };

        if *debug { println!("Found: {}: capacity {}, durability {}, flavor {}, texture {}, calories {}",
            name, capacity, durability, flavor, texture, calories)}

        ingredient_list.push(Ingredient::new(name, capacity, durability, flavor, texture, calories));

    }

    ingredient_list
}

#[derive(Debug)]
struct Ingredient {
    _name: String,
    capacity: isize,
    durability: isize,
    flavor: isize,
    texture: isize,
    calories: usize,
}

impl Ingredient {
    fn new(
        name: String,
        capacity: isize,
        durability: isize,
        flavor: isize,
        texture: isize,
        calories: usize
    ) -> Ingredient {

        Ingredient { _name: name, capacity, durability, flavor, texture, calories }
    }
}