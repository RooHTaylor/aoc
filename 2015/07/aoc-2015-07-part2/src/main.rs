use shared::*;
use std::collections::HashMap;
use std::collections::VecDeque;

const DEBUG: bool = false;

fn main() {
    let args = parse_args();
    
    let file_contents = load_input_file(&args[1]);

    build_computer(&file_contents);
}

// Build the circuit using the instructions
// Create a list of wires and a queue of gates to process.
// As we parse each instruction we will update values and then loop through all 
// pending gates for any that may now be solved.
fn build_computer(instructions: &str) {
    let mut wires: HashMap<String, Option<u16>> = HashMap::new();
    let mut pending_gates: VecDeque<Gate> = VecDeque::new();

    for instruction in instructions.lines() {
        // Parse all the values in one instruction and generate the gate
        parse_instruction(&instruction, &mut pending_gates);
        // loop through pending gates to try to solve
        process_gate_queue(&mut wires, &mut pending_gates);
    }

    // Flush the remaining gates after all instructions have been read.
    while pending_gates.len() > 0 {
        // loop through pending gates to try to solve
        process_gate_queue(&mut wires, &mut pending_gates);
    }

    let a = match wires.get("a") {
        Some(n) => n.unwrap(),
        None => 0
    };

    println!("First a is {}", a);

    let mut wires: HashMap<String, Option<u16>> = HashMap::new();
    wires.insert("b".to_string(), Some(a));

    for instruction in instructions.lines() {
        if instruction.ends_with(" -> b") {
            println!("We skipped setting b: {}", instruction);
            continue;
        }
        // Parse all the values in one instruction and generate the gate
        parse_instruction(&instruction, &mut pending_gates);
        // loop through pending gates to try to solve
        process_gate_queue(&mut wires, &mut pending_gates);
    }

    // Flush the remaining gates after all instructions have been read.
    while pending_gates.len() > 0 {
        // loop through pending gates to try to solve
        process_gate_queue(&mut wires, &mut pending_gates);
    }

    let a = match wires.get("a") {
        Some(n) => n.unwrap(),
        None => 0
    };

    println!("The value of a is {}", a);
    
}

// Process the queue of pending gates for any that can be solved.
fn process_gate_queue(
    wires: &mut HashMap<String, Option<u16>>,
    pending_gates: &mut VecDeque<Gate>
) {

    for _ in 0..pending_gates.len() {
        if DEBUG { println!("{} gates in the queue", pending_gates.len())}
        // Get the next gate to process
        let gate = match pending_gates.pop_front() {
            Some(g) => g,
            None => {
                return;
            }
        };

        if DEBUG { println!("trying to solve gate:\n{:#?}", gate); }

        let output = execute_gate(wires, &gate);
        // If there was no output, we don't have the required inputs yet. Push the 
        // gate into a queue to be processed later.
        match output {
            Some(o) => {
                if DEBUG { println!("solved: {}", o); }
                wires.insert(gate.output.to_string(), output);
            },
            None => {
                if DEBUG { println!("unable to solve. pushing to back"); }
                pending_gates.push_back(gate);
            }
        }
    }
}

// Execute a gate action
fn execute_gate(
    wires: &mut HashMap<String, Option<u16>>,
    gate: &Gate
) -> Option<u16> {
    // Create or retreive the input
    let input1 = retrieve_or_create_wire(wires, &gate.input1);
    if DEBUG { println!("input 1: {:#?}", input1)}
    // If we're expecting to have a second input, also create or retrieve it
    let input2 = match gate.input2 {
        Some(ref n) => {
            retrieve_or_create_wire(wires, n)
        },
        None => None,
    };
    if DEBUG { println!("input 2: {:#?}", input2); }

    if DEBUG { println!("action: {:#?}", gate.action); }

    // Generate our output by performing the gate action on our inputs
    let output = match gate.action {
        GateAction::DIRECT => {
            match input1 {
                Some(v) => {
                    Some(v)
                },
                None => {
                    if DEBUG { println!("Can't solve because we don't have {}", gate.input1); }
                    None
                },
            }
        },
        GateAction::NOT => {
            match input1 {
                Some(v) => {
                    Some(!v)
                },
                None => {
                    if DEBUG { println!("Can't solve because we don't have {}", gate.input1); }
                    None
                },
            }
        },
        GateAction::OR => {
            match input1 {
                Some(v1) => {
                    match input2 {
                        Some(v2) => {
                            Some(v1 | v2)
                        },
                        None => {
                            if DEBUG { println!("Can't solve because we don't have {:#?}", gate.input2); }
                            None
                        },
                    }
                },
                None => {
                    if DEBUG { println!("Can't solve because we don't have {}", gate.input1); }
                    None
                },
            }
        },
        GateAction::AND => {
            match input1 {
                Some(v1) => {
                    match input2 {
                        Some(v2) => {
                            Some(v1 & v2)
                        },
                        None => {
                            if DEBUG { println!("Can't solve because we don't have {:#?}", gate.input2); }
                            None
                        },
                    }
                },
                None => {
                    if DEBUG { println!("Can't solve because we don't have {}", gate.input1); }
                    None
                },
            }
        },
        GateAction::LSHIFT => {
            match input1 {
                Some(v1) => {
                    match input2 {
                        Some(v2) => {
                            Some(v1 << v2)
                        },
                        None => {
                            if DEBUG { println!("Can't solve because we don't have {:#?}", gate.input2); }
                            None
                        },
                    }
                },
                None => {
                    if DEBUG { println!("Can't solve because we don't have {}", gate.input1); }
                    None
                },
            }
        },
        GateAction::RSHIFT => {
            match input1 {
                Some(v1) => {
                    match input2 {
                        Some(v2) => {
                            Some(v1 >> v2)
                        },
                        None => {
                            if DEBUG { println!("Can't solve because we don't have {:#?}", gate.input2); }
                            None
                        },
                    }
                },
                None => {
                    if DEBUG { println!("Can't solve because we don't have {}", gate.input1); }
                    None
                },
            }
        },
    };

    output
}

// Parse one line of instruction
// Identify the inputs, output, and action. Create wires that don't already exist 
// and execute the gate action if possible.
fn parse_instruction(
    instruction: &str,
    pending_gates: &mut VecDeque<Gate>
) {

    // Initialize the gate
    let mut gate = Gate {
        action: GateAction::DIRECT,
        input1: String::new(),
        input2: None,
        output: String::new(),
    };

    if DEBUG { println!("processing: {instruction}")}

    // Split the instruction into the input side and the output side. Get the output
    // name.
    let parts: Vec<&str> = instruction.split(" -> ").collect();
    if parts.len() != 2 {
        eprintln!("The instruction seems to be malformed!: {instruction}");
        return;
    }
    gate.output.push_str(parts[1].trim());

    //if DEBUG { println!("found output wire: {}", gate.output)}

    // Parse the input side.
    // Input side should be either
    //      1 -> i1
    //      2 -> NOT i1
    //      3 -> i1 ACTION i2
    let parts: Vec<&str> = parts[0].trim().split(" ").collect();
    // Assign the input names and action based on the number of input components
    match parts.len() {
        // Only an input
        1 => {
            gate.input1.push_str(parts[0].trim());
        },
        // Should be a NOT with an input
        2 if parts[0].starts_with("NOT") => {
            gate.action = GateAction::NOT;
            gate.input1.push_str(parts[1].trim());
        },
        // Should be a dual input action
        3 => {
            gate.action = match parts[1].trim() {
                "OR" => GateAction::OR,
                "AND" => GateAction::AND,
                "LSHIFT" => GateAction::LSHIFT,
                "RSHIFT" => GateAction::RSHIFT,
                _ => {
                    eprintln!("The instruction seems to be malformed! {instruction}");
                    return;
                },
            };
            gate.input1.push_str(parts[0].trim());
            gate.input2 = Some(parts[2].trim().to_string());
        },
        _ => {
            eprintln!("The instruction seems to be malformed!: {instruction}");
            return;
        },
    }
    //if DEBUG { println!("found input wire: {}", gate.input1)}
    //if DEBUG { println!("found input wire 2: {:#?}", gate.input2)}
    //if DEBUG { println!("gate action: {:#?}", gate.action)}
    
    // Push the gate into the queue to be processes
    pending_gates.push_back(gate);
}

// Retreive a Wire_value from the HashMap, or create it if it doesn't exist yet.
// Return the reference to the new entry
// Note we use .entry() here instead of .get() to avoid lifetime issues of creating
// a temporary value that goes out of scope.
fn retrieve_or_create_wire<'a>(
    wires: &'a mut HashMap<String, Option<u16>>,
    name: &str
) -> Option<u16> {
    let wire= wires
        .entry(name.to_string())
        .or_insert(
            if name.parse::<u16>().is_ok() {
                Some(name.parse::<u16>().unwrap())
            } else {
                None
            }
        );

    wire.clone()
}

#[derive(Debug, Copy, Clone)]
enum GateAction {
    DIRECT,
    OR,
    AND,
    LSHIFT,
    RSHIFT,
    NOT,
}

#[derive(Debug)]
struct Gate {
    action: GateAction,
    input1: String,
    input2: Option<String>,
    output: String,
}
