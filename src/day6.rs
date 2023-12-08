#![allow(clippy::needless_return)]

use timer::profile;

pub fn run_day6(inputs: &str) {
    profile! {
        let day6_1 = day6_1(inputs);
        println!("Day 6-1: {day6_1}");
    }

    profile! {
        let day6_2 = day6_2(inputs);
        println!("Day 6-2: {day6_2}");
    }
}

struct Races {
    times: Vec<usize>,
    distances: Vec<usize>,
}

impl Races {
    fn new(inputs: &str) -> Self {
        let mut times: Vec<usize> = Vec::new();
        let mut distances: Vec<usize> = Vec::new();

        for line in inputs.lines() {
            if times.is_empty() {
                times = line
                    .split_ascii_whitespace()
                    .filter(|&x| !x.starts_with('T'))
                    .map(|x| x.parse().unwrap())
                    .collect();
            } else {
                distances = line
                    .split_ascii_whitespace()
                    .filter(|&x| !x.starts_with('D'))
                    .map(|x| x.parse().unwrap())
                    .collect();
            }
        }

        return Races { times, distances };
    }
}

fn day6_1(inputs: &str) -> usize {
    let races = Races::new(inputs);

    let mut combos: Vec<usize> = Vec::new();
    for (itime, time) in races.times.iter().enumerate() {
        let target = races.distances[itime];
        for charge in 0..time + 1 {
            let traveled = charge * (time - charge);
            if traveled > target {
                let min = charge;
                let max = time - min + 1;
                combos.push(max - min);
                break;
            }
        }
    }

    return combos.iter().product();
}

fn day6_2(inputs: &str) -> usize {
    let races = Races::new(inputs);
    let mut time = String::new();
    let mut target = String::new();

    // there is probably a better way to do this
    races
        .times
        .iter()
        .for_each(|&x| time.push_str(x.to_string().as_str()));
    races
        .distances
        .iter()
        .for_each(|&x| target.push_str(x.to_string().as_str()));
    let time: usize = time.parse().unwrap();
    let target: usize = target.parse().unwrap();

    for charge in 0..time + 1 {
        let traveled = charge * (time - charge);
        if traveled > target {
            let min = charge;
            let max = time - min + 1;
            return max - min;
        }
    }

    panic!("Didn't find valid race combinations");
}
