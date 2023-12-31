#![allow(clippy::needless_return)]

use timer::profile;

pub fn run_day{day}(inputs: &str) {
    profile! {
        let day{day}_1 = day{day}_1(inputs);
        println!("Day {day}-1: {day{day}_1}");
    }

    profile! {
        let day{day}_2 = day{day}_2(inputs);
        println!("Day {day}-2: {day{day}_2}");
    }
}

fn day{day}_1(inputs: &str) -> usize {
    return 0;
}

fn day{day}_2(inputs: &str) -> usize {
    return 0;
}