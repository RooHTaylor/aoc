use std::env;
use std::process;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};
use std::any::type_name;
use serde_json;

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

    let mut json_string: String = String::new();

    for line in lines.map_while(Result::ok) {
        // ignore empty lines
        if line.is_empty() {
            continue;
        }

        json_string.push_str(line.trim());
    }

    let json_data: serde_json::Value = serde_json::from_str(&json_string).unwrap();

    let sum: i64 = find_sum(&json_data);

    println!("Sum is {}", sum);

}

fn find_sum(json_data: &serde_json::Value) -> i64 {

    let mut sum: i64 = 0;

    if json_data.is_object() {
        for i in json_data.as_object().unwrap() {
            let (key, v) = i;
            println!("{}: {}", key, v);
            if v.is_object() || v.is_array() {
                let tsum: i64 = find_sum(&v);
                //println!("Adding {}", tsum);
                sum += tsum;
            } else if v.is_i64() {
                println!("{} is number!", v);
                let tsum: i64 = v.as_i64().unwrap();
                println!("Adding {}", tsum);
                sum += tsum;
            } else if v.is_string() && v.as_str().unwrap() == "red" {
                println!("Red value found!");
                return 0;
            }
        }
    } else if json_data.is_array() {
        for i in json_data.as_array().unwrap() {
            let v = i;
            println!("{}", v);
            if v.is_object() || v.is_array() {
                let tsum: i64 = find_sum(&v);
                //println!("Adding {}", tsum);
                sum += tsum;
            } else if v.is_i64() {
                println!("{} is number!", v.as_i64().unwrap());
                let tsum: i64 = v.as_i64().unwrap();
                println!("Adding {}", tsum);
                sum += tsum;
            }
        }
    } else {
        if json_data.is_i64() {
            //let tsum: i64 = json_data.as_i64().unwrap();
            //println!("Adding {}.", tsum);
            //sum += tsum;
        }
    }

    sum
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
