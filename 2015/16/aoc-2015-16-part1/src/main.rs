use shared::*;

fn main() {
    let args = parse_args();
    let mut debug: bool = false;
    if args.len() == 3 && &args[2] == "--debug" {
        debug = true;
    }
    
    let file_contents = load_input_file(&args[1]);
    if debug { println!("{file_contents}"); }

    let mut sue_to_find: Sue = Sue::new(
        0, 
        Some(3),
        Some(7),
        Some(2),
        Some(3),
        Some(0),
        Some(0),
        Some(5),
        Some(3),
        Some(2),
        Some(1),
    );

    let known_sue_list = parse_sue_data(&file_contents);

    if debug { println!("{:#?}", known_sue_list); }

    // Step through each Sue and see if all the known values match
    for sue in known_sue_list {
        if sue.children != None && sue.children != sue_to_find.children {
            continue;
        }
        if sue.cats != None && sue.cats != sue_to_find.cats {
            continue;
        }
        if sue.samoyeds != None && sue.samoyeds != sue_to_find.samoyeds {
            continue;
        }
        if sue.pomeranians != None && sue.pomeranians != sue_to_find.pomeranians {
            continue;
        }
        if sue.akitas != None && sue.akitas != sue_to_find.akitas {
            continue;
        }
        if sue.vizslas != None && sue.vizslas != sue_to_find.vizslas {
            continue;
        }
        if sue.goldfish != None && sue.goldfish != sue_to_find.goldfish {
            continue;
        }
        if sue.trees != None && sue.trees != sue_to_find.trees {
            continue;
        }
        if sue.cars != None && sue.cars != sue_to_find.cars {
            continue;
        }
        if sue.perfumes != None && sue.perfumes != sue_to_find.perfumes {
            continue;
        }

        sue_to_find.id = sue.id;
        break;
    }

    println!("Found a Sue with all attributes either matching or unknown: {}", sue_to_find.id);
}

// Parse each line of input to load all the known Data about each Sue
// and load it into a Vec<Sue>.  Unknown values get a None.
fn parse_sue_data(input: &str) -> Vec<Sue> {
    let mut known_sue_list: Vec<Sue> = Vec::new();

    for line in input.lines() {
        if line.is_empty() { continue; }

        // Split by the first : which should yeild 
        let (sueid, attributes) = match line.split_once(": ") {
            Some(t) => t,
            None => {
                eprintln!("The line could not be parsed! {line}");
                continue;
            }
        };

        // Pull the id out of the first string
        let id = match sueid.trim().split_once(" ") {
            Some(s) => {
                match s.1.trim().parse::<usize>() {
                    Ok(n) => n,
                    Err(_) => {
                        eprintln!("Id is not a number! {}", s.1);
                        continue;
                    }
                }
            },
            None => {
                eprintln!("Could not extract the id from {sueid}");
                continue;
            }
        };

        // Split attributes into individual parts to be processed
        let attributes: Vec<&str> = attributes.trim().split(",").collect();
        if attributes.len() < 1 {
            eprintln!("There were not enough attributes! {line}");
            continue;
        }

        let mut children: Option<usize> = None;
        let mut cats: Option<usize> = None;
        let mut samoyeds: Option<usize> = None;
        let mut pomeranians: Option<usize> = None;
        let mut akitas: Option<usize> = None;
        let mut vizslas: Option<usize> = None;
        let mut goldfish: Option<usize> = None;
        let mut trees: Option<usize> = None;
        let mut cars: Option<usize> = None;
        let mut perfumes: Option<usize> = None;

        // Loop through the attributes
        for attr in attributes {

            let (name, amount) = match attr.trim().split_once(":") {
                Some(t) => (t.0.trim(), t.1.trim()),
                None => {
                    eprintln!("Improperly formatted attribute!");
                    continue;
                }
            };
            let amount = match amount.parse::<usize>() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("Invalid number for attribute {name}");
                    continue;
                }
            };

            match name {
                "children" => {
                    children = Some(amount);
                },
                "cats" => {
                    cats = Some(amount);
                },
                "samoyeds" => {
                    samoyeds = Some(amount);
                },
                "pomeranians" => {
                    pomeranians = Some(amount);
                },
                "akitas" => {
                    akitas = Some(amount);
                },
                "vizslas" => {
                    vizslas = Some(amount);
                },
                "goldfish" => {
                    goldfish = Some(amount);
                },
                "trees" => {
                    trees = Some(amount);
                },
                "cars" => {
                    cars = Some(amount);
                },
                "perfumes" => {
                    perfumes = Some(amount);
                },
                _ => {
                    eprintln!("Invalid attribute {name} found!");
                    continue;
                }
            }
        }

        let new_sue = Sue::new(id, children, cats, samoyeds, pomeranians, akitas, vizslas, goldfish, trees, cars, perfumes);
        known_sue_list.push(new_sue);
    }

    known_sue_list
}

#[derive(Debug)]
struct Sue {
    id: usize,
    children: Option<usize>,
    cats: Option<usize>,
    samoyeds: Option<usize>,
    pomeranians: Option<usize>,
    akitas: Option<usize>,
    vizslas: Option<usize>,
    goldfish: Option<usize>,
    trees: Option<usize>,
    cars: Option<usize>,
    perfumes: Option<usize>,
}

impl Sue {
    fn new(
        id: usize,
        children: Option<usize>,
        cats: Option<usize>,
        samoyeds: Option<usize>,
        pomeranians: Option<usize>,
        akitas: Option<usize>,
        vizslas: Option<usize>,
        goldfish: Option<usize>,
        trees: Option<usize>,
        cars: Option<usize>,
        perfumes: Option<usize>
    ) -> Sue {

        Sue {
            id,
            children,
            cats,
            samoyeds,
            pomeranians,
            akitas,
            vizslas,
            goldfish,
            trees,
            cars,
            perfumes,
        }
    }
}