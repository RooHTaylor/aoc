use std::env;
use std::process;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    // Read filename from args
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <inputfilename>", &args[0]);
        process::exit(1);
    }
    let filename = Path::new(&args[1]);
    // Open file and read lines into a buffered reader
    let file = match File::open(&filename) {
        Ok(fp) => fp,
        Err(_) => {
            println!("Unable to open file! {}", filename.to_str().unwrap());
            process::exit(2);
        },
    };
    let lines = io::BufReader::new(file).lines();

    let mut current_password = String::new();

    for line in lines.map_while(Result::ok) {
        // skip empty lines
        if line .is_empty() {
            continue;
        }

        // input should only be one line
        current_password.push_str(line.trim());
        break;
    }

    if current_password.is_empty() {
        println!("The current password cannot be empty!");
        process::exit(3);
    }

    let mut iters: usize = 0;
    let mut found_passwords: usize = 0;
    loop {

        if current_password.len() != 8 {
            println!("Oops! Password not 8 characters!");
            break;
        }

        if iters % 10000 == 0 {
            println!("Iteration: {} current_password: {:?}",iters, current_password);
        }

        // Collect each char as bytes to make it easy to add 1 to
        let mut password_bytes: Vec<u8> = current_password.bytes().collect();

        //println!("{:?}", password_bytes);

        // Grab the last char as the current char and increment it. If it's > z, set
        // it to a and step up to increment that char. If it is not > z then break.
        let mut last_index: usize = password_bytes.len() - 1;
        let mut keep_looping: bool = false;
        loop {
            password_bytes[last_index] = match password_bytes[last_index] {
                b'z' => {
                    if last_index == 0 {
                        password_bytes.insert(0, b'a');
                        // our index position changes after inserting the new element
                        last_index = 1;
                    } else {
                        keep_looping = true;
                    }
                    b'a'
                },
                // skip i, l, and o
                b'h' | b'k' | b'n' => {
                    password_bytes[last_index] + 2
                },
                _ => {
                    password_bytes[last_index] + 1
                },
            };
            if !keep_looping {
                break;
            } else {
                last_index -= 1;
                keep_looping = false;
            }
        }

        // Check for the first condition.  If there are 3 bytes in a row that
        // increment only by 1, then the first condition is met.
        let mut condition1: bool = false;
        // Check for second condition. Password cannot contain i o or l
        let mut condition2: bool = true;
        // Check for the third condition. If there are 2 sets of 2 chars in a 
        // row that are the same then the third condition is met.
        let mut condition3: bool = false;
        let mut char_matches: Vec<u8> = Vec::new();
        let mut next_possible_char_match: usize = 0;
        for i in 0..password_bytes.len() {
            // Valid passwords may not contain i o or l. Increment these characters
            // if found
            match password_bytes[i] {
                b'i' | b'o' | b'l' => {
                    condition2 = false;
                },
                _ => ()
            }
            if
                i > 2 &&
                password_bytes[i] - 1 == password_bytes[i-1] &&
                password_bytes[i-1] - 1 == password_bytes[i-2]
            {
                condition1 = true;
            }
            if
                i > 1 &&
                password_bytes[i] == password_bytes[i-1] &&
                i > next_possible_char_match
            {
                char_matches.push(password_bytes[i]);
                next_possible_char_match = i + 1;
            }
        }

        if char_matches.len() >= 2 {
            condition3 = true;
        }

        current_password = String::from_utf8(password_bytes).unwrap();

        iters += 1;
        if condition1 && condition2 && condition3 {
            println!("{} is a valid password!", current_password);
            found_passwords += 1;
            if found_passwords == 2 {
                break;
            }
        }
    }
}
