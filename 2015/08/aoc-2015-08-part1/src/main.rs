use shared::*;

const DEBUG: bool = false;

fn main() {
    let args = parse_args();
    
    let file_contents = load_input_file(&args[1]);

    let (total_chars, total_mem) = parse_strings(&file_contents);

    println!("Total memory usage: {}", total_mem);
    println!("Total string length: {}", total_chars);
    println!("Answer: {}", total_mem - total_chars);
}


fn parse_strings(input: &str) -> (usize, usize) {
    let mut total_chars: usize = 0;
    let mut total_mem: usize = 0;

    let mut escaped: bool = false;
    let mut hex: u8 = 0;

    for line in input.lines() {
        if DEBUG {println!("Examining: {line}"); }
        for char in line.trim().chars() {
            if DEBUG {println!("Working char: {char}"); }
            // Every char takes up memory.
            total_mem += 1;
            if DEBUG {println!("Total memory: {total_mem}"); }

            match char {
                // If we encounter a \ we're either already escaped and it's 1 
                // char or start escaping.
                '\\' => {
                    if DEBUG {println!("Caught a \\"); }
                    if escaped {
                        if DEBUG {println!("We're already escaped"); }
                        total_chars += 1;
                        if DEBUG {println!("Total chars: {total_chars}"); }
                        escaped = false;
                        if DEBUG {println!("No longer escaped."); }
                    } else {
                        escaped = true;
                        if DEBUG {println!("We're not escaped! Now escaping!"); }
                    }
                },
                // If we encounter an x, it's either an x or there was a \ and
                // it should be hex
                'x' => {
                    if escaped {
                        hex += 1;
                        if DEBUG {println!("Caught an x while escaped! Hex char {hex}"); }
                    } else {
                        total_chars += 1;
                        if DEBUG {println!("Total chars: {total_chars}"); }
                    }
                },
                // If we encounter a " it only counts as char if we're escaped
                '"' => {
                    if escaped {
                        if DEBUG {println!("Caught an x while escaped! Hex char {hex}"); }
                        total_chars += 1;
                        escaped = false;
                    }
                }
                _ => {
                    // Are we escaped? Must have been examining a hex escape.
                    // Check if we're done.
                    if escaped && hex > 0 {
                        if hex < 3 {
                            // We haven't counted 3 chars yet. Should still be hex.
                            if !char.is_digit(16) {
                                eprintln!("We encountered a {char} where there should be valid hex! {line}");
                            }
                            hex += 1;
                            if DEBUG {println!("We're escaped! This should be hex. Hex char {hex}"); }
                            if hex == 3 {
                                // We've counted 3 hex (1 char)
                                escaped = false;
                                hex = 0;
                                total_chars += 1;
                                if DEBUG {println!("We've counted 3 hex chars. Done escaping!"); }
                                if DEBUG {println!("Total chars: {total_chars}"); }
                            }
                        } else {
                            eprintln!("We're escaping, but hex didn't get reset somewhere!");
                        }
                    } else {
                        total_chars += 1;
                        if DEBUG {println!("Total chars: {total_chars}"); }
                    }
                },
            }
        }
    }

    (total_chars, total_mem)
}