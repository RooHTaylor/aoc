use std::{
    env,
    process,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut debug: bool = false;
    let mut input: usize = 1;
    let mut i: usize = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--debug" => {
                debug = true;
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

    let house_number = find_lowest_house(debug, input);
    println!("The lowest house number to get at least {} presents is: {}", input, house_number);

}

fn find_lowest_house(debug: bool, target: usize) -> usize {

    let mut elf_counts: Vec<usize> = Vec::new();
    let mut n = 1;
    loop {
        if debug || n % 10000 == 0 { println!("House {n}"); }
        let sum = calculate_presents(debug, n, &mut elf_counts);
        if debug { println!("House {n} got {sum} presents"); }
        if sum >= target {
            return n;
        }
        n += 1;
    }
}

fn calculate_presents(debug:bool, n: usize, elf_counts: &mut Vec<usize>) -> usize {
    let mut sum = 0;
    let sqrt_n = (n as f64).sqrt() as usize;

    for elf in 1..=sqrt_n {
        if n % elf == 0 {
            if debug { println!("Elf {elf} is a divisor of house {n}"); }
            
            if elf >= elf_counts.len() {
                elf_counts.resize_with(elf + 1, || 0);
            }

            if elf_counts[elf] < 50 {
                sum += elf*11;
                elf_counts[elf] += 1;
                if debug { println!("Elf {elf} has now seen {} houses.", elf_counts[elf]); }
            } else {
                if debug { println!("Elf {elf} has already visited {} houses.", elf_counts[elf]); }
            }

            if elf != n / elf {
                let inverse_elf = n / elf;
                if debug { println!("Inverse elf is {inverse_elf}"); }
                if inverse_elf >= elf_counts.len() {
                    elf_counts.resize_with(inverse_elf + 1, || 0);
                }

                if elf_counts[inverse_elf] < 50 {
                    sum += inverse_elf*11;
                    elf_counts[inverse_elf] += 1;
                    if debug { println!("Elf {inverse_elf} has now seen {} houses.", elf_counts[inverse_elf]); }
                } else {
                    if debug { println!("Elf {inverse_elf} has already visited {} houses.", elf_counts[inverse_elf]); }
                }
            }
        }
    }
    sum
}