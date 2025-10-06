use std::{
    env,
    process,
    fs,
    path::Path,
};

// Read the arguments and confirm an input file was provided
// The function allows for an optional 3rd argument, --debug
// 
// # Exits - code 1
//
// This function will exit the process if the number of arguments are incorrect
pub fn parse_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || (args.len() == 3 && &args[2] != "--debug") {
        eprintln!("Usage: {} <input file> [--debug]", &args[0]);
        eprintln!("       OR cargo run -- <input file> [--debug]");
        process::exit(1);
    }

    args
}

// Check if the input file exists and is a file. Open it and return the contents
// as a String.
//
// # Exits - code 1
//
// This function will exit the process if the file does not exist or is not a file
pub fn load_input_file(filename: &str) -> String {
    let filepath= Path::new(filename);

    if !filepath.exists() || !filepath.is_file() {
        eprintln!("The input file you provided either doesn't exist, or isn't a file.");
        process::exit(1);
    }

    let file_contents = match fs::read_to_string(filename) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("There was a problem reading from the input file: {err}");
            process::exit(1);
        }
    };

    file_contents
}

// Read the arguments and confirm an input file was provided or the number of iterations was provided
// The function allows for an optional 3rd argument, --debug
// 
// # Exits - code 1
//
// This function will exit the process if the number of arguments are incorrect
pub fn parse_args_iterations() -> Vec<String> {
    let args: Vec<String> = env::args().collect();

    if
        args.len() < 2
        || (args.len() == 3 && !(&args[2] == "--debug" || args[2].parse::<usize>().is_ok())) 
        || (args.len() == 4 && !(&args[2] == "--debug" || args[2].parse::<usize>().is_ok()) && !(&args[3] == "--debug" || args[3].parse::<usize>().is_ok()))
    {
        eprintln!("Usage: {} <input file> [<iterations>] [--debug]", &args[0]);
        eprintln!("       OR cargo run -- <input file> [iterations] [--debug]");
        process::exit(1);
    }

    args
}