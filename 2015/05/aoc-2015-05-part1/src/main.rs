use shared::*;

fn main() {
    let args = parse_args();
    
    let file_contents = load_input_file(&args[1]);

    let num_nice_strings = count_nice_strings(&file_contents);

    println!("The total number of nice strings is {num_nice_strings}");
}

fn count_nice_strings(contents: &str) -> usize {

    let mut nice_count: usize = 0;

    for line in contents.lines() {
        // Skip empty lines
        if line.trim().is_empty() { continue; }

        if is_nice(&line) {
            nice_count += 1;
        }
    }

    nice_count
}

fn is_nice(input_string: &str) -> bool {

    if input_string.trim().is_empty() {
        return false
    }

    let input_string_chars: Vec<char> = input_string.trim().chars().collect();

    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let forbidden_strings = ["ab", "cd", "pq", "xy"];

    let mut vowel_count:u8 = 0;
    let mut double_letters = false;

    // Initialize last_c to a null-byte.
    let mut last_c: char = '\0';

    for c in input_string_chars {
        let mut last_two = String::with_capacity(2);
        last_two.push(last_c);
        last_two.push(c);

        // Fast fail if we have a forbidden string
        if forbidden_strings.contains(&last_two.as_str()) {
            return false
        }

        if c == last_c {
            double_letters = true;
        }

        if vowels.contains(&c) {
            vowel_count += 1;
        }

        last_c = c;
    }

    vowel_count >= 3 && double_letters
}