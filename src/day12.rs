#![allow(clippy::needless_return)]

use timer::profile;

pub fn run_day12(inputs: &str) {
    profile! {
        let day12_1 = day12_1(inputs);
        println!("Day 12-1: {day12_1}");
    }

    profile! {
        let day12_2 = day12_2(inputs);
        println!("Day 12-2: {day12_2}");
    }
}

fn day12_1(inputs: &str) -> usize {
    let mut total = 0;
    for line in inputs.lines() {
        let split: Vec<&str> = line.split_ascii_whitespace().collect();
        let arrangement = split[0];
        let groupings: Vec<usize> = split[1].split(',').map(|x| x.parse().unwrap()).collect();

        // singular combination
        if groupings.iter().sum::<usize>() + groupings.len() - 1 == arrangement.len() {
            total += 1;
            continue;
        }

        let mut good = 0;
        let mut bad = 0;
        let mut unkown = 0;
        arrangement.chars().for_each(|c| match c {
            '.' => good += 1,
            '#' => bad += 1,
            '?' => unkown += 1,
            _ => (),
        });
        let spaces = good + bad + unkown;
        println!("g {}, b {}, u {}, t {}", good, bad, unkown, spaces);
    }

    return total;
}

fn day12_2(_inputs: &str) -> usize {
    return 0;
}

// TODO: make math lib
#[allow(dead_code)]
fn ncr(n: usize, r: usize) -> usize {
    let n_fact = factorial(n);
    let r_fact = factorial(r);
    let n_r_fact = factorial(n - r);

    return n_fact / (r_fact * n_r_fact).max(1);
}
#[allow(dead_code)]
fn factorial(n: usize) -> usize {
    if n == 0 {
        return 1;
    }
    let mut ret = n;
    for i in 1..n {
        ret *= n - i;
    }

    return ret;
}
