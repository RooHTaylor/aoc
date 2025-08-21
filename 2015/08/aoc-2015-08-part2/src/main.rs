use shared::*;

const DEBUG: bool = false;

fn main() {
    let args = parse_args();
    
    let file_contents = load_input_file(&args[1]);

    let (total_chars, total_mem) = parse_strings(&file_contents);

    println!("Total memory usage: {}", total_mem);
    println!("Total string length: {}", total_chars);
    println!("Answer: {}", total_chars - total_mem);
}


fn parse_strings(input: &str) -> (usize, usize) {
    let mut total_encoded: usize = 0;
    let mut total_mem: usize = 0;

    for line in input.lines() {
        if DEBUG {println!("Examining: {line}"); }
        // Add "" around line
        total_encoded += 2;
        for char in line.trim().chars() {
            if DEBUG {println!("Working char: {char}"); }
            // Every char takes up memory.
            total_mem += 1;
            if DEBUG {println!("Total memory: {total_mem}"); }

            match char {
                '\\' => {
                    // Add a backslash in-front of \
                    total_encoded += 2;
                },
                '"' => {
                    // Add a backslash in-front of "
                    total_encoded += 2;
                }
                _ => {
                    total_encoded += 1;
                },
            }
        }
    }

    (total_encoded, total_mem)
}