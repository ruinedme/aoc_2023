#![allow(clippy::needless_return)]

use timer::profile;

pub fn run_day2(inputs: &str) {
    profile! {
        let day2_1 = day2_1(inputs);
        println!("Day 2-1: {day2_1}");
    }

    profile! {
        let day2_2 = day2_2(inputs);
        println!("Day 2-2: {day2_2}");
    }
}

const MAX_RED: usize = 12;
const MAX_GREEN: usize = 13;
const MAX_BLUE: usize = 14;

#[derive(Debug)]
struct CubeSet {
    red: usize,
    green: usize,
    blue: usize,
}

#[derive(Debug)]
struct Game {
    id: usize,
    sets: Vec<CubeSet>,
}

impl Game {
    fn new(input: &str) -> Self {
        let id_start = input.find(|c: char| c.is_ascii_digit()).unwrap();
        let id_end = input.find(':').unwrap();
        let id: usize = input[id_start..id_end].parse().unwrap();
        let offset = id_end + 2;
        let mut sets: Vec<CubeSet> = Vec::new();

        for game_sets in input[offset..].split(';') {
            let mut cube_set: CubeSet = CubeSet {
                red: 0,
                green: 0,
                blue: 0,
            };
            for color in game_sets.split(',') {
                let t: Vec<&str> = color.trim().split(' ').collect();

                match t[1].trim() {
                    "red" => cube_set.red = t[0].trim().parse().unwrap(),
                    "blue" => cube_set.blue = t[0].trim().parse().unwrap(),
                    "green" => cube_set.green = t[0].trim().parse().unwrap(),
                    x => panic!("Found invalid color: {}", x),
                }
            }
            sets.push(cube_set);
        }

        #[allow(clippy::needless_return)]
        return Game { id, sets };
    }
}

fn day2_1(inputs: &str) -> usize {
    let mut total = 0;
    for line in inputs.lines() {
        let game = Game::new(line);
        let impossible_games: Vec<&CubeSet> = game
            .sets
            .iter()
            .filter(|&x| x.red > MAX_RED || x.green > MAX_GREEN || x.blue > MAX_BLUE)
            .collect();

        if impossible_games.is_empty() {
            total += game.id;
        }
    }

    return total;
}

fn day2_2(inputs: &str) -> usize {
    let mut total = 0;

    for line in inputs.lines() {
        let game = Game::new(line);
        let red = game
            .sets
            .iter()
            .max_by(|x, y| x.red.cmp(&y.red))
            .unwrap()
            .red;
        let green = game
            .sets
            .iter()
            .max_by(|x, y| x.green.cmp(&y.green))
            .unwrap()
            .green;
        let blue = game
            .sets
            .iter()
            .max_by(|x, y| x.blue.cmp(&y.blue))
            .unwrap()
            .blue;

        total += red * green * blue;
    }

    #[allow(clippy::needless_return)]
    return total;
}
