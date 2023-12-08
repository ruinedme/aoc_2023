#![allow(clippy::needless_return)]

use timer::profile;

pub fn run_day1(inputs: &str) {
    profile! {
        let day1_1 = day1_1(inputs);
        println!("Day 1-1: {day1_1}");
    }

    profile! {
        let day1_2 = day1_2(inputs);
        println!("Day 1-2: {day1_2}");
    }
}

fn day1_1(inputs: &str) -> usize {
    return inputs
        .lines()
        .map(|line| {
            line.to_string()
                .chars()
                .filter(|c| c.is_ascii_digit())
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .map(|vec| 10 * vec.first().unwrap() + vec.last().unwrap())
        .sum();
}

fn day1_2(inputs: &str) -> usize {
    return inputs
        .lines()
        .map(|line| {
            line.to_string()
                .replace("zero", "zero0zero")
                .replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine")
                .chars()
                .filter(|c| c.is_ascii_digit())
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .map(|vec| 10 * vec.first().unwrap() + vec.last().unwrap())
        .sum();
}
