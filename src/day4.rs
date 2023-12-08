#![allow(clippy::needless_return)]

use std::collections::{HashMap, HashSet};

use timer::profile;

pub fn run_day4(inputs: &str) {
    profile! {
        let day4_1 = day4_1(inputs);
        println!("Day 4-1: {day4_1}");
    }

    profile! {
        let day4_2 = day4_2(inputs);
        println!("Day 4-2: {day4_2}");
    }
}

fn parse(inputs: &str) -> Vec<HashSet<&str>> {
    let mut cards: Vec<HashSet<&str>> = Vec::new();
    for line in inputs.lines() {
        let numbers: Vec<&str> = line.split(": ").collect();
        let numbers: Vec<&str> = numbers[1].trim().split(" | ").collect();
        let winning_numbers: HashSet<&str> = numbers[0]
            .trim()
            .split_ascii_whitespace()
            .map(|x| x.trim())
            .collect();
        let numbers_you_have: HashSet<&str> = numbers[1]
            .trim()
            .split_ascii_whitespace()
            .map(|x| x.trim())
            .collect();
        let matching_numbers: HashSet<&str> = winning_numbers
            .intersection(&numbers_you_have)
            .copied()
            .collect();
        cards.push(matching_numbers);
    }

    return cards;
}

fn day4_1(inputs: &str) -> usize {
    return parse(inputs)
        .iter()
        .filter(|&x| !x.is_empty())
        .fold(0, |acc, x| acc + 2i32.pow(x.len() as u32 - 1) as usize);
}

fn day4_2(inputs: &str) -> usize {
    let cards = parse(inputs);
    let mut copies: HashMap<usize, usize> = cards
        .iter()
        .enumerate()
        .map(|(card, _)| (card, 1))
        .collect();

    for (i, n) in cards.iter().enumerate() {
        //process each copy of current card
        for _ in 0..*copies.get(&i).unwrap() {
            //add the copy
            for j in i + 1..i + n.len() + 1 {
                copies.entry(j).and_modify(|x| *x += 1).or_insert(1);
            }
        }
    }
    return copies.iter().map(|x| x.1).sum();
}
