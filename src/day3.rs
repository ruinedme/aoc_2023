use std::collections::HashMap;

use grid::Grid;
use timer::profile;

pub fn run_day3(inputs: &String) {
    profile! {
        let day3_1 = day3_1(&inputs);
        println!("Day 3-1: {day3_1}");
    }

    profile! {
        let day3_2 = day3_2(&inputs);
        println!("Day 3-2: {day3_2}");
    }
}

fn day3_1(inputs: &String) -> usize {
    let mut total = 0;
    let grid = Grid::new(inputs);

    for (row, y) in grid.map.iter().enumerate() {
        let mut buff: Vec<u8> = Vec::new();
        let mut is_part_num = false;

        for (col, &x) in y.iter().enumerate() {
            match x {
                b'0'..=b'9' => {
                    buff.push(x);
                    grid.get_all_neighbors((row, col)).iter().for_each(|&n| {
                        match grid.map[n.0][n.1] {
                            b'.' | b'0'..=b'9' => (),
                            _ => {
                                is_part_num = true;
                            }
                        }
                    });
                }
                _ => {
                    if !buff.is_empty() {
                        if is_part_num {
                            let s: usize =
                                String::from_utf8(buff.clone()).unwrap().parse().unwrap();
                            total += s;
                            is_part_num = false;
                        }
                        buff.clear();
                    }
                }
            }
        }
        // handle numbers at the end of a line
        if !buff.is_empty() {
            if is_part_num {
                let s: usize = String::from_utf8(buff.clone()).unwrap().parse().unwrap();
                total += s;
            }
        }
    }
    return total;
}

fn day3_2(inputs: &String) -> usize {
    let grid = Grid::new(inputs);
    let mut gears: HashMap<(usize, usize), Vec<usize>> = HashMap::new();

    for (row, y) in grid.map.iter().enumerate() {
        let mut buff: Vec<u8> = Vec::new();
        let mut is_gear = false;
        let mut gear_pos: (usize, usize) = (0, 0);
        for (col, &x) in y.iter().enumerate() {
            match x {
                b'0'..=b'9' => {
                    buff.push(x);
                    grid.get_all_neighbors((row, col)).iter().for_each(|&n| {
                        match grid.map[n.0][n.1] {
                            // b'.'|b'0'..=b'9' => (),
                            b'*' => {
                                is_gear = true;
                                gear_pos = n;
                            }
                            _ => (),
                        }
                    });
                }
                _ => {
                    if !buff.is_empty() {
                        if is_gear {
                            let s: usize =
                                String::from_utf8(buff.clone()).unwrap().parse().unwrap();
                            gears
                                .entry(gear_pos)
                                .and_modify(|x| x.push(s))
                                .or_insert(vec![s]);
                            is_gear = false;
                        }
                        buff.clear();
                    }
                }
            }
        }
        // handle numbers at the end of a line
        if !buff.is_empty() {
            if is_gear {
                let s: usize = String::from_utf8(buff.clone()).unwrap().parse().unwrap();
                gears
                    .entry(gear_pos)
                    .and_modify(|x| x.push(s))
                    .or_insert(vec![s]);
            }
        }
    }

    return gears
        .iter()
        .filter(|&x| x.1.len() == 2)
        .map(|x| x.1.iter().product::<usize>())
        .sum();
}
