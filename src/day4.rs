use std::collections::HashMap;

use timer::profile;

pub fn run_day4(inputs: &String) {
    profile! {
        let day4_1 = day4_1(&inputs);
        println!("Day 4-1: {day4_1}");
    }

    profile! {
        let day4_2 = day4_2(&inputs);
        println!("Day 4-2: {day4_2}");
    }
}

fn day4_1(inputs: &String) -> usize {
    let mut total = 0;
    for line in inputs.lines() {
        let numbers: Vec<&str> = line.split(": ").collect();
        let numbers: Vec<&str> = numbers[1].trim().split(" | ").collect();
        let winning_numbers: Vec<&str> = numbers[0]
            .trim()
            .split(" ")
            .filter(|&x| !x.is_empty())
            .map(|x| x.trim())
            .collect();
        let numbers_you_have: Vec<&str> = numbers[1]
            .trim()
            .split(" ")
            .filter(|&x| !x.is_empty())
            .map(|x| x.trim())
            .collect();
        let matching_numbers: Vec<&str> = winning_numbers
            .iter()
            .filter(|&x| numbers_you_have.contains(&x))
            .map(|&x| x)
            .collect();

        if !matching_numbers.is_empty() {
            let x = 2i32.pow(matching_numbers.len() as u32 - 1) as usize;
            total += x;
        }
    }
    return total;
}

fn day4_2(inputs: &String) -> usize {
    let mut matches: Vec<usize> = Vec::new();
    let mut copies: HashMap<usize, usize> = HashMap::new();

    for (card, line) in inputs.lines().enumerate() {
        copies.entry(card).and_modify(|x| *x += 1).or_insert(1);

        let numbers: Vec<&str> = line.split(": ").collect();
        let numbers: Vec<&str> = numbers[1].trim().split(" | ").collect();
        let winning_numbers: Vec<&str> = numbers[0]
            .trim()
            .split(" ")
            .filter(|&x| !x.is_empty())
            .map(|x| x.trim())
            .collect();
        let numbers_you_have: Vec<&str> = numbers[1]
            .trim()
            .split(" ")
            .filter(|&x| !x.is_empty())
            .map(|x| x.trim())
            .collect();
        let matching_numbers: Vec<&str> = winning_numbers
            .iter()
            .filter(|&x| numbers_you_have.contains(&x))
            .map(|&x| x)
            .collect();

        matches.push(matching_numbers.len());
    }

    // for each instance of card (i), add 1 copy to i+1..i+n+1 where n is the number of matches for i
    for (i, &n) in matches.iter().enumerate() {
        //process each copy of current card
        for _ in 0..*copies.get(&i).unwrap() {
            //add the copy
            for j in i + 1..i + n + 1 {
                copies.entry(j).and_modify(|x| *x += 1).or_insert(1);
            }
        }
    }
    return copies.iter().map(|x| x.1).sum();
}
