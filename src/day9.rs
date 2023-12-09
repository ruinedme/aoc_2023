#![allow(clippy::needless_return)]

use timer::profile;

pub fn run_day9(inputs: &str) {
    profile! {
        let day9_1 = day9_1(&inputs);
        println!("Day 9-1: {day9_1}");
    }

    profile! {
        let day9_2 = day9_2(&inputs);
        println!("Day 9-2: {day9_2}");
    }
}

fn day9_1(inputs: &str) -> isize {
    let mut total = 0;
    for line in inputs.lines() {
        let history: Vec<isize> = line
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let n = find_next_sequence(&history);

        total += n;
    }

    return total;
}

fn day9_2(inputs: &str) -> isize {
    let mut total = 0;
    for line in inputs.lines() {
        let mut history: Vec<isize> = line
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        history.reverse();
        let n = find_next_sequence(&history);
        total += n;
    }

    return total;
}

fn find_next_sequence(sequence: &Vec<isize>) -> isize {
    let next_sequence: Vec<isize> = sequence
        .iter()
        .enumerate()
        .filter(|(i, _)| *i > 0)
        .map(|(i, a)| a - sequence[i - 1])
        .collect();

    if next_sequence.iter().all(|x| *x == 0) {
        return sequence.last().unwrap() + next_sequence.last().unwrap();
    }

    return sequence.last().unwrap() + find_next_sequence(&next_sequence);
}
