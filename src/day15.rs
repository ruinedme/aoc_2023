#![allow(clippy::needless_return)]

use timer::profile;

pub fn run_day15(inputs: &str) {
    profile! {
        let day15_1 = day15_1(inputs);
        println!("Day 15-1: {day15_1}");
    }

    profile! {
        let day15_2 = day15_2(inputs);
        println!("Day 15-2: {day15_2}");
    }
}

fn day15_1(inputs: &str) -> usize {
    return inputs.split(',').fold(0, |acc, s| acc + hash(s));
}

fn day15_2(inputs: &str) -> usize {
    let instructions: Vec<Instruction> = inputs.split(',').map(Instruction::new).collect();
    let mut boxes: Vec<Vec<Instruction>> = vec![Vec::new(); 256];
    for i in instructions {
        let box_index = hash(&i.label);
        let lens_index = boxes[box_index].iter().position(|x| x.label == i.label);
        match i.operator {
            Operators::Insert => {
                if lens_index.is_some() {
                    boxes[box_index][lens_index.unwrap()] = i;
                } else {
                    boxes[box_index].push(i);
                }
            }
            Operators::Remove => {
                if let Some(v) = lens_index {
                    boxes[box_index].remove(v);
                }
            }
            Operators::None => (),
        }
    }

    // probably can be condensed, but nested folds make my brain hurt
    let mut total = 0;
    for (i, b) in boxes.iter().enumerate() {
        let mut focusing_power = 0;
        if !b.is_empty() {
            b.iter().enumerate().for_each(|(j, lens)| {
                focusing_power += (1 + i) * (1 + j) * lens.focal_len.unwrap() as usize;
            });
        }
        total += focusing_power;
    }
    return total;
}

fn hash(input: &str) -> usize {
    let mut t = 0;
    for c in input.chars() {
        t += c as usize;
        t *= 17;
        t %= 256;
    }

    return t;
}

#[derive(Debug, Clone)]
enum Operators {
    Insert,
    Remove,
    None,
}

#[derive(Debug, Clone)]
struct Instruction {
    label: String,
    operator: Operators,
    focal_len: Option<u8>,
}

impl Instruction {
    fn new(input: &str) -> Self {
        let mut label = String::new();
        let mut operator = Operators::None;
        let mut focal_len = None;
        for c in input.chars() {
            match c {
                'a'..='z' => label.push(c),
                '-' => operator = Operators::Remove,
                '=' => operator = Operators::Insert,
                '1'..='9' => focal_len = Some(c as u8 - b'0'),
                _ => (),
            }
        }
        return Self {
            label,
            operator,
            focal_len,
        };
    }
}
