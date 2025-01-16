use std::env;
use std::process;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

#[derive(Debug)]
enum Operations {
    NOP,
    NOT,
    AND,
    OR,
    LSHIFT,
    RSHIFT,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Wire {
    name: String,
    value: u16,
    set: bool,
}

#[derive(Debug)]
struct Gate {
    input1: String,
    input2: Option<String>,
    operation: Operations,
    output: String,
}

fn main() {
    // Open and read the provided file
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <inputfilename>", args[0]);
        process::exit(1);
    }
    let filename = Path::new(&args[1]);
    let file = match File::open(&filename) {
        Ok(fp) => fp,
        Err(_) => {
            println!("Failed to open file! {}", &args[1]);
            process::exit(2);
        }
    };
    let lines = io::BufReader::new(file).lines();

    // Wires will contain a list of all the wires and their values. The key will
    // be the wire name.
    let mut wires: HashMap<String, Wire> = HashMap::new();

    // For part 2 we know the value of wire b is going to be the value of wire a
    // after running part 1. We can just insert wire b with that value here, and 
    // it will be overridden.
    let b_wire = Wire {
        name: String::from("b"),
        value: 956,
        set: true,
    };
    wires.insert(String::from(&b_wire.name), b_wire);

    // As gates are discovered, they may not be able to be processed yet (i.e. the
    // input wires are not set.) Push them into a Vec to be processed later, once
    // more wires have been set.
    let mut pending_gates: Vec<Gate> = Vec::new();

    // Loop over lines from BufReader
    for line in lines.map_while(Result::ok) {
        // Skip empty lines
        if line.is_empty(){
            continue;
        }
    
        // Parse the input to get the operation and the output
        let line_parts: Vec<&str> = line.trim().split(" -> ").collect();
        if line_parts.len() != 2 {
            println!("Unable to parse line! {}", &line);
            process::exit(3);
        }

        // Parse the operation parts and create the gate
        let operation_parts: Vec<&str> = line_parts[0].split(" ").collect();
        let gate: Gate = match operation_parts.len() {
            1 => Gate {
                input1: String::from(operation_parts[0]),
                input2: None,
                operation: Operations::NOP,
                output: String::from(line_parts[1]),
            },
            2 => Gate {
                input1: String::from(operation_parts[1]),
                input2: None,
                operation: Operations::NOT,
                output: String::from(line_parts[1]),
            },
            3 => Gate {
                input1: String::from(operation_parts[0]),
                input2: Some(String::from(operation_parts[2])),
                operation: match operation_parts[1] {
                    "AND" => Operations::AND,
                    "OR" => Operations::OR,
                    "LSHIFT" => Operations::LSHIFT,
                    "RSHIFT" => Operations::RSHIFT,
                    _ => {
                        println!("Invalid operation! {}", operation_parts[1]);
                        process::exit(4);
                    },
                },
                output: String::from(line_parts[1]),
            },
            _ => {
                println!("Unable to process operation string! {}", line_parts[0]);
                process::exit(5);
            },
        };

        // Load the output wire. It it's not in the HashMap then create it and 
        // insert it into the HashMap
        let mut output: Wire = match wires.get(&gate.output) {
            Some(o) => o.clone(),
            None => {
                let new_wire = Wire {
                    name: String::from(&gate.output),
                    value: 0,
                    set: false,
                };
                wires.insert(String::from(&gate.output), new_wire);
                wires.get(&gate.output).unwrap().clone()
            },
        };
        // Output can only be set once, so if it's already set we have a problem
        if output.set {
            println!("Output already set on {}", output.name);
            continue;
        }

        // Load the input1 wire. If it's not in the HashMap then create it and 
        // insert it into the HashMap. We treat straight assignments as wires and
        // assign them the u16 value of their name
        let input1: Wire = match wires.get(&gate.input1) {
            Some(i) => i.clone(),
            None => {
                let new_wire = Wire {
                    name: String::from(&gate.input1),
                    // Try to convert to u16. If successful then it must be a
                    // direct assignment. Set the value and set=true
                    value: match gate.input1.parse() {
                        Ok(v) => v,
                        Err(_) => 0,
                    },
                    set: match gate.input1.parse::<u16>() {
                        Ok(_) => true,
                        Err(_) => false,
                    },
                };
                wires.insert(String::from(&gate.input1), new_wire);
                wires.get(&gate.input1).unwrap().clone()
            },
        };

        // Load the input2 wire. If it's not in the HashMap then create it and 
        // insert it into the HashMap. We treat straight assignments as wires and
        // assign them the u16 value of their name. (LSHIFT and RSHIFT take numbers
        // as their second input)
        let input2: Option<Wire> = match gate.operation {
            Operations::OR |
            Operations::AND | 
            Operations::LSHIFT |
            Operations::RSHIFT => {
                // Load/Create input2 wire used for these gates
                match wires.get(gate.input2.as_ref().unwrap()) {
                    Some(w) => Some(w.clone()),
                    None => {
                        let new_wire = Wire {
                            name: String::from(gate.input2.as_ref().unwrap()),
                            // try to convert to u16. If successful then it must be
                            // a direct assignment. set the value and set=true
                            value: match gate.input2.as_ref().unwrap().parse::<u16>() {
                                Ok(n) => n,
                                Err(_) => 0,
                            },
                            set: match gate.input2.as_ref().unwrap().parse::<u16>() {
                                Ok(_) => true,
                                Err(_) => false,
                            },
                        };
                        wires.insert(
                            String::from(gate.input2.as_ref().unwrap()),
                            new_wire
                        );
                        Some(
                            wires.get(gate.input2.as_ref().unwrap()).unwrap().clone()
                        )
                    },
                }
            },
            _ => None,
        };

        // If either input1 or input2 are not set, then we can't proceed with this
        // gate. Push it to the pending queue to be processed later.
        if !input1.set || 
            (input2 != Option::None && !input2.as_ref().unwrap().set)
        {
            pending_gates.push(gate);
            continue;
        }

        // Both inputs must be set (or not used) if we're here, so we can process
        // the gate.
        match gate.operation {
            Operations::NOP => {
                output.value = input1.value;
            },
            Operations::NOT => {
                output.value = !input1.value;
            },
            Operations::OR => {
                output.value = input1.value | input2.unwrap().value;
            },
            Operations::AND => {
                output.value = input1.value & input2.unwrap().value;
            },
            Operations::LSHIFT => {
                output.value = input1.value << input2.unwrap().value;
            },
            Operations::RSHIFT => {
                output.value = input1.value >> input2.unwrap().value;
            },
        }
        output.set = true;
        wires.insert(String::from(&output.name), output);

        // Since we've updated a wire, let's process any the pending gates as one
        // may have changes
        if pending_gates.len() > 0 {
            (wires, pending_gates) = process_pending_gates(wires, pending_gates);
        }
    }

    // Process any remaining gates
    while pending_gates.len() > 0 {
        println!("{:#?} {:#?}", pending_gates, wires);
        (wires, pending_gates) = process_pending_gates(wires, pending_gates);
    }

    println!("{:#?}", wires);

    match wires.get("a") {
        Some(w) => {
            println!("Wire a is: {:#?}", w);
        }
        None => {
            println!("No wire a!");
        }
    }
}

fn process_pending_gates(
    mut wires: HashMap<String, Wire>,
    mut pending_gates: Vec<Gate>
) -> (HashMap<String, Wire>, Vec<Gate>) {

    // Loop over the pending gates. Pull out the index to update the keep Vec for
    // the retain function later.
    loop {
        // Nothing to loop over
        if pending_gates.len() < 1 {
            break;
        }

        let mut keep_looping = false;

        // While we loop through pending_gates we keep track of the ones we want to
        // remove. At the end we retain only the ones with true in the same spot in
        // the keep vec
        let mut keep: Vec<bool> = vec![true; pending_gates.len()];

        for (i, g) in pending_gates.iter().enumerate() {
            // Load the wires. They should all exist already
            let mut output: Wire = match wires.get(&g.output) {
                Some(w) => w.clone(),
                None => {
                    println!("Impossible! This wire should exist! {}", &g.output);
                    process::exit(6);
                },
            };
            let input1: Wire = match wires.get(&g.input1) {
                Some(w) => w.clone(),
                None => {
                    println!("Impossible! This wire should exist! {}", &g.input1);
                    process::exit(6);
                },
            };
            // Based on operation, load remaining wire and run operation to update 
            // output wire. Push to pending_gates if inputs are not net.
            let input2: Option<Wire> = match g.operation {
                Operations::OR |
                Operations::AND | 
                Operations::LSHIFT |
                Operations::RSHIFT => {
                    // Load/Create input2 wire used for these gates
                    match wires.get(g.input2.as_ref().unwrap()) {
                        Some(w) => Some(w.clone()),
                        None => {
                            println!(
                                "Impossible! This wire should exist! {}",
                                g.input2.as_ref().unwrap()
                            );
                            process::exit(7);
                        },
                    }
                },
                _ => None,
            };

            // If either input1 or input2 are not set, then we can't proceed with
            // this gate.
            if !input1.set ||
                (input2 != Option::None && !input2.as_ref().unwrap().set) 
            {
                continue;
            }

            // Both inputs must be set (or not used) if we're here, so we can
            // process the gate.
            match g.operation {
                Operations::NOP => {
                    output.value = input1.value;
                },
                Operations::NOT => {
                    output.value = !input1.value;
                },
                Operations::OR => {
                    output.value = input1.value | input2.unwrap().value;
                },
                Operations::AND => {
                    output.value = input1.value & input2.unwrap().value;
                },
                Operations::LSHIFT => {
                    output.value = input1.value << input2.unwrap().value;
                },
                Operations::RSHIFT => {
                    output.value = input1.value >> input2.unwrap().value;
                },
            }
            output.set = true;
            wires.insert(String::from(&output.name), output);
            keep[i] = false;

            // We updated a wire, so run the loop at least one more time
            keep_looping = true;
        }

        // remove the updated wires from the pending_gates
        let mut iter = keep.iter();
        pending_gates.retain(|_| *iter.next().unwrap());

        // If a wire was updated, keep_looping will be true so we loop and process
        // all the gates again. If no wire was updated, keep_looping will be false
        // and the loop will break
        if !keep_looping {
            break;
        }
    }

    return (wires, pending_gates)
}
