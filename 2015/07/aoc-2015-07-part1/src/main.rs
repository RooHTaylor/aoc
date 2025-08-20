use shared::*;
use std::collections::HashMap;

fn main() {
    let args = parse_args();
    
    let file_contents = load_input_file(&args[1]);

    build_computer(&file_contents);
}

fn build_computer(instructions: &str) {
    let mut wires: HashMap<String, Wire> = HashMap::new();
    let mut pending_gates: Vec<Gate> = Vec::new();

    for instruction in instructions.lines() {
        // Parse all the values in one instruction. Solve the gate if possible
        parse_instruction(&instruction, &mut wires, &mut pending_gates);
        // loop through pending gates to try to solve
    }

    println!("{wires:#?}");
}

// Parse one line of instruction
// Identify the inputs, output, and action. Create wires that don't already exist 
// and execute the gate action if possible.
fn parse_instruction(instruction: &str, wires: &mut HashMap<String, Wire>, pending_gates: &mut Vec<Gate>) {

    // Initialize the gate
    let mut gate = Gate {
        action: GateAction::DIRECT,
        input1: String::new(),
        input2: None,
        output: String::new(),
    };

    // Split the instruction into the input side and the output side. Get the output
    // name.
    let parts: Vec<&str> = instruction.split(" -> ").collect();
    if parts.len() != 2 {
        eprintln!("The instruction seems to be malformed!: {instruction}");
        return;
    }
    gate.output.push_str(parts[1].trim());

    // Parse the input side.
    // Input side should be either
    //      1 -> i1
    //      2 -> NOT i1
    //      3 -> i1 ACTION i2
    let parts: Vec<&str> = parts[0].trim().split(" ").collect();
    // Assign the input names and action based on the number of input components
    match parts.len() {
        1 => {
            gate.input1.push_str(parts[0].trim());
        },
        2 if parts[0].starts_with("NOT") => {
            gate.action = GateAction::NOT;
            gate.input1.push_str(parts[1].trim());
        },
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

    // Create or retrieve input1
    let input1 = retrieve_or_create_wire(wires, &gate.input1);
    let input1_value: Option<u16> = if input1.value_set { Some(input1.value) } else { None };
    let input2 = match gate.input2 {
        Some(ref n) => {
            Some(retrieve_or_create_wire(wires, n))
        },
        None => None,
    };
    let input2_value: Option<u16> = match input2 {
        Some(w) => {
            if w.value_set { Some(w.value) } else { None }
        },
        None => None,
    };

    let output_value = match gate.action {
        GateAction::DIRECT => {
            input1_value
        },
        GateAction::NOT => {
            match input1_value {
                Some(v) => {
                    Some(!v)
                },
                None => None,
            }
        },
        GateAction::OR => {
            match input1_value {
                Some(v1) => {
                    match input2_value {
                        Some(v2) => {
                            Some(v1 | v2)
                        },
                        None => None,
                    }
                },
                None => None,
            }
        },
        GateAction::AND => {
            match input1_value {
                Some(v1) => {
                    match input2_value {
                        Some(v2) => {
                            Some(v1 & v2)
                        },
                        None => None,
                    }
                },
                None => None,
            }
        },
        GateAction::LSHIFT => {
            match input1_value {
                Some(v1) => {
                    match input2_value {
                        Some(v2) => {
                            Some(v1 << v2)
                        },
                        None => None,
                    }
                },
                None => None,
            }
        },
        GateAction::RSHIFT => {
            match input1_value {
                Some(v1) => {
                    match input2_value {
                        Some(v2) => {
                            Some(v1 >> v2)
                        },
                        None => None,
                    }
                },
                None => None,
            }
        },
    };

    match output_value {
        Some(v) => {
            let output = retrieve_or_create_wire(wires, &gate.output);
            output.value = v;
            output.value_set = true;
        },
        None => {
            pending_gates.push(gate);
        }
    }

}

// Retreive a Wire from the HashMap, or create it if it doesn't exist yet.
// Return the reference to the new entry
// Note we use .entry() here instead of .get() to avoid lifetime issues of creating
// a temporary value that goes out of scope.
fn retrieve_or_create_wire<'a>(wires: &'a mut HashMap<String, Wire>, name: &str) -> &'a mut Wire {
    let wire= wires
        .entry(name.to_string())
        .or_insert(
            Wire {
                name: name.to_string(),
                value_set: name.parse::<u16>().is_ok(),
                value: if name.parse::<u16>().is_ok() { name.parse::<u16>().unwrap() } else { 0 },
            }
        ); 

    wire
}

#[derive(Debug, Clone)]
struct Wire {
    name: String,
    value_set: bool,
    value: u16,
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
