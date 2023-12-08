#![allow(clippy::needless_return)]

use std::collections::HashMap;

use timer::profile;

pub fn run_day7(inputs: &str) {
    profile! {
        let day7_1 = day7_1(inputs);
        println!("Day 7-1: {day7_1}");
    }

    profile! {
        let day7_2 = day7_2(inputs);
        println!("Day 7-2: {day7_2}");
    }
}

const CARDS: [u8; 13] = [
    b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'T', b'J', b'Q', b'K', b'A',
];

#[derive(PartialEq, PartialOrd, Debug)]
enum HandType {
    None,
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

#[derive(Debug)]
struct Round {
    hand: String,
    bid: usize,
    hand_type: HandType,
    j_is_wild: bool,
}

impl PartialEq for Round {
    fn eq(&self, other: &Self) -> bool {
        return self.hand == other.hand && self.hand_type == other.hand_type;
    }
}

impl PartialOrd for Round {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_type.partial_cmp(&other.hand_type) {
            Some(core::cmp::Ordering::Equal) => {
                for i in 0..5 {
                    let mut a = self.hand.as_bytes()[i];
                    let mut b = other.hand.as_bytes()[i];
                    if a == b {
                        continue;
                    }
                    a = Round::card_strength(a, self.j_is_wild);
                    b = Round::card_strength(b, other.j_is_wild);
                    if a == b {
                        continue;
                    }
                    return a.partial_cmp(&b);
                }
                return None;
            }
            ord => return ord,
        }
    }
}

impl Round {
    fn new(line: &str, wildcards: bool) -> Self {
        let split: Vec<&str> = line.split_ascii_whitespace().collect();
        let mut card_map: HashMap<u8, usize> = HashMap::new();
        split[0].as_bytes().iter().for_each(|&x| {
            card_map.entry(x).and_modify(|e| *e += 1).or_insert(1);
        });
        #[allow(unused_assignments)]
        let mut hand_type = HandType::None;
        let max = card_map
            .iter()
            .filter(|&x| !(wildcards && *x.0 == b'J' && card_map.keys().len() > 1))
            .max_by(|x, &y| x.1.cmp(y.1))
            .map(|x| *x.1)
            .unwrap();
        if wildcards && card_map.contains_key(&b'J') && card_map.keys().len() > 1 {
            let t = max + card_map.get(&b'J').unwrap();
            hand_type = match t {
                5 => HandType::FiveOfKind,
                4 => HandType::FourOfKind,
                3 => {
                    if card_map.len() - 1 == 2 {
                        HandType::FullHouse
                    } else {
                        HandType::ThreeOfKind
                    }
                }
                2 => HandType::OnePair,
                _ => HandType::None,
            }
        } else {
            hand_type = match card_map.keys().len() {
                1 => HandType::FiveOfKind,
                2 => {
                    //FourKind or Full
                    if max == 4 {
                        HandType::FourOfKind
                    } else {
                        HandType::FullHouse
                    }
                }
                3 => {
                    //ThreeKind or 2 Pair
                    if max == 3 {
                        HandType::ThreeOfKind
                    } else {
                        HandType::TwoPair
                    }
                }
                4 => HandType::OnePair,
                5 => HandType::HighCard,
                _ => HandType::None,
            };
        }

        return Self {
            hand: String::from(split[0]),
            bid: split[1].parse().expect("Expected Valid Number"),
            hand_type,
            j_is_wild: wildcards,
        };
    }

    fn card_strength(card: u8, wildcards: bool) -> u8 {
        for (i, &c) in CARDS.iter().enumerate() {
            if c == card {
                if wildcards && card == b'J' {
                    return 0;
                } else {
                    return (i + 1) as u8;
                }
            }
        }
        panic!("Invalid Card Value {}", card.to_ascii_uppercase());
    }
}

fn day7_1(inputs: &str) -> usize {
    let mut rounds: Vec<Round> = inputs.lines().map(|x| Round::new(x, false)).collect();
    rounds.sort_by(|a, b| a.partial_cmp(b).unwrap());

    return rounds
        .iter()
        .enumerate()
        .map(|(i, x)| (i + 1) * x.bid)
        .sum();
}

fn day7_2(inputs: &str) -> usize {
    let mut rounds: Vec<Round> = inputs.lines().map(|x| Round::new(x, true)).collect();
    rounds.sort_by(|a, b| a.partial_cmp(b).unwrap());

    return rounds
        .iter()
        .enumerate()
        .map(|(i, x)| (i + 1) * x.bid)
        .sum();
}
