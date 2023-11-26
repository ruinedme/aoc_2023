use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Missing Input File.");
        println!("Usage: {} [1-25] input_file", args[0]);
        return;
    }

    let day: &u8 = &args[1].parse().expect("Day must be between 1 and 25 inclusive.");

    let inputs = fs::read_to_string(&args[2]).unwrap();

    match day {
        1..=25 => println!("Not Implemented."),
        _ => println!("Invalid Day.")
    }
}
