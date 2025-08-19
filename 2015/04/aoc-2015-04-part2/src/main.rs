use shared::*;
use md5;

fn main() {
    let args = parse_args();
    
    let file_contents = load_input_file(&args[1]);

    let num_leading_zeros = 6;
    let number = mine_advent_coins(&file_contents, num_leading_zeros);

    println!("The smallest number that produces a hash with {num_leading_zeros} leading zeros is {number}")
}

// Increment a number by 1 and append it to the end of your secret_key string
// Calculate the MD5 hex and examine to see how many leading 0s are present.
// This will find the smallest number that will result in a hash with x 0s
fn mine_advent_coins(secret_key: &str, num_leading_zeros: usize) -> usize {
    let mut number: usize = 0;

    loop {
        let mut string = String::from(secret_key);
        string.push_str(&number.to_string());

        let digest = md5::compute(&string);

        // We cannot examine bytes. Need to examine the hex string itself.
        let digest_hex_string = format!("{:x}", digest);

        let leading_zeros_match = digest_hex_string
            .chars()
            .take(num_leading_zeros)
            .all(|char| char == '0');

        if leading_zeros_match {
            break;
        }

        number += 1;
    }

    number
}