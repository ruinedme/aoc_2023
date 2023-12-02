use aoc_2023::*;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Missing Input File");
        println!("Usage: {} [1-25] input_file", args[0]);
        return;
    }

    let day: &u8 = &args[1].parse().expect("Day must be between 1 and 25 inclusive.");

    let inputs = fs::read_to_string(&args[2]).unwrap();

    match day {
        1 => day1::run_day1(&inputs),
        2 => day2::run_day2(&inputs),
        // 3 => day3::run_day3(&inputs),
        // 4 => day4::run_day4(&inputs),
        // 5 => day5::run_day5(&inputs),
        // 6 => day6::run_day6(&inputs),
        // 7 => day7::run_day7(&inputs),
        // 8 => day8::run_day8(&inputs),
        // 9 => day9::run_day9(&inputs),
        // 10 => day10::run_day10(&inputs),
        // 11 => day11::run_day11(&inputs),
        // 12 => day12::run_day12(&inputs),
        // 13 => day13::run_day13(&inputs),
        // 14 => day14::run_day14(&inputs),
        // 15 => day15::run_day15(&inputs),
        // 16 => day16::run_day16(&inputs),
        // 17 => day17::run_day17(&inputs),
        // 18 => day18::run_day18(&inputs),
        // 19 => day19::run_day19(&inputs),
        // 20 => day20::run_day20(&inputs),
        // 21 => day21::run_day21(&inputs),
        // 22 => day22::run_day22(&inputs),
        // 23 => day23::run_day23(&inputs),
        // 24 => day24::run_day24(&inputs),
        // 25 => day25::run_day25(&inputs),
        _ => panic!("Invalid Day"),
    }
}
