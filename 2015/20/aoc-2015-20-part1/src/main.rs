use std::{
    env,
    process
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut _debug: bool = false;
    let mut input: usize = 1;
    let mut i: usize = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--debug" => {
                _debug = true;
            },
            "-i" => {
                if i+1 >= args.len() {
                    eprintln!("Not enough arguments provided for input");
                    process::exit(1);
                }
                input = match args[i+1].parse::<usize>() {
                    Ok(n) => n,
                    Err(_) => input,
                };
                i += 1;
            },
            _ => {
                eprintln!("Usage: {} [-i <input>] [--debug]", args[0]);
            }
        }
        i += 1;
    }

    let house_number = find_lowest_house(input);
    println!("The lowest house number to get at least {} presents is: {}", input, house_number);

}

fn find_lowest_house(target: usize) -> usize {
    let target_divisor_sum = target / 10;

    let mut n = 1;
    loop {
        let divisor_sum = sum_of_divisors(n);
        if divisor_sum >= target_divisor_sum {
            return n;
        }
        n += 1;
    }
}

fn sum_of_divisors(n: usize) -> usize {
    let mut sum = 0;
    let sqrt_n = (n as f64).sqrt() as usize;

    for i in 1..=sqrt_n {
        if n % i == 0 {
            sum += i; // Add the divisor
            if i != n / i {
                sum += n / i; // Add the corresponding divisor
            }
        }
    }
    sum
}