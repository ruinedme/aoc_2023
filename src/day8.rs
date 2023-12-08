use std::collections::HashMap;

use timer::profile;

pub fn run_day8(inputs: &String) {
    profile! {
        let day8_1 = day8_1(&inputs);
        println!("Day 8-1: {day8_1}");
    }

    profile! {
        let day8_2 = day8_2(&inputs);
        println!("Day 8-2: {day8_2}");
    }
}

#[derive(Debug, Clone)]
struct Node {
    left: Vec<u8>,
    right: Vec<u8>,
}

#[derive(Debug)]
struct Map {
    move_sequence: Vec<u8>,
    nodes: HashMap<Vec<u8>, Node>,
}

impl Map {
    fn new<'a>(inputs: &String) -> Self {
        let mut lines = inputs.lines();
        let move_sequence: Vec<u8> = lines
            .next()
            .unwrap()
            .as_bytes()
            .iter()
            .map(|x| *x)
            .collect();
        lines.next();

        let nodes: HashMap<Vec<u8>, Node> = lines
            .map(|x| {
                let split: Vec<&str> = x.split_ascii_whitespace().collect();
                (
                    split[0].as_bytes().iter().map(|x| *x).collect(),
                    Node {
                        left: split[2][1..4].as_bytes().iter().map(|x| *x).collect(),
                        right: split[3][0..3].as_bytes().iter().map(|x| *x).collect(),
                    },
                )
            })
            .collect();

        return Map {
            move_sequence,
            nodes,
        };
    }

    /// final return value must be >= min_moves
    fn get_moves(&self, start_node: &Vec<u8>) -> usize {
        let mut count = 1;
        let mut current_node = self.nodes.get(start_node).unwrap();
        for mv in self.move_sequence.iter().cycle() {
            match mv {
                b'L' => {
                    if current_node.left.last().unwrap() == &b'Z' && count > 0 {
                        break;
                    }
                    current_node = self.nodes.get(&current_node.left).unwrap();
                }
                b'R' => {
                    if current_node.right.last().unwrap() == &b'Z' && count > 0 {
                        break;
                    }
                    current_node = self.nodes.get(&current_node.right).unwrap()
                }
                _ => panic!("Invalid Sequence {}", mv.to_ascii_uppercase()),
            }
            count += 1;
        }

        return count;
    }
}

fn day8_1(inputs: &String) -> usize {
    let map = Map::new(inputs);
    let start_node: Vec<Vec<u8>> = map
        .nodes
        .iter()
        .filter(|x| x.0.last().unwrap() == &b'A')
        .map(|x| x.0.clone())
        .collect();
    return map.get_moves(&start_node[0]);
}

// length of move sequence is a prime number
// all move sequences will be a factor of the move sequence length
// answer is LCM(n0,n1,..) where n is the minimum move sequence of each starting node
fn day8_2(inputs: &String) -> usize {
    let map = Map::new(inputs);

    let current_nodes: Vec<(Vec<u8>, Node)> = map
        .nodes
        .iter()
        .filter(|&x| x.0.last().unwrap() == &b'A')
        .map(|x| (x.0.clone(), x.1.clone()))
        .collect();

    let mut moves: Vec<usize> = current_nodes.iter().map(|x| map.get_moves(&x.0)).collect();
    if moves.len() == 1 {
        return moves[0];
    }

    let gcf = map.move_sequence.len();
    let mut lcm = (moves.pop().unwrap() * moves.pop().unwrap()) / gcf;
    // handle if moves only had 2 starting locations like in the example
    if moves.len() == 0 {
        return lcm * gcf;
    }
    while moves.len() > 0 {
        lcm = (lcm * moves.pop().unwrap()) / gcf;
    }
    return lcm;
}
