#![allow(clippy::needless_return)]

use grid::Grid;
use timer::profile;

pub fn run_day13(inputs: &str) {
    profile! {
        let day13_1 = day13_1(inputs);
        println!("Day 13-1: {day13_1}");
    }

    profile! {
        let day13_2 = day13_2(inputs);
        println!("Day 13-2: {day13_2}");
    }
}

fn day13_1(inputs: &str) -> usize {
    let mut total = 0;
    let mut pattern = String::new();

    for line in inputs.lines() {
        if !line.is_empty() {
            pattern.push_str(line);
            pattern.push('\n');
        } else {
            let grid = Grid::new(&pattern);
            pattern = String::new();
            let mut s = check_vertical(&grid, false);
            if s == 0 {
                s = check_horizontal(&grid, false);
            }

            total += s;
        }
    }

    // handle last pattern in file
    let grid = Grid::new(&pattern);
    let mut s = check_vertical(&grid, false);
    if s == 0 {
        s = check_horizontal(&grid, false);
    }
    total += s;

    return total;
}

// 34610 -- too low
fn day13_2(inputs: &str) -> usize {
    let mut total = 0;
    let mut pattern = String::new();

    for line in inputs.lines() {
        if !line.is_empty() {
            pattern.push_str(line);
            pattern.push('\n');
        } else {
            let grid = Grid::new(&pattern);
            // grid.print_map();
            // println!();
            pattern = String::new();
            let mut s = check_horizontal(&grid, true);
            if s == 0 {
                s = check_vertical(&grid, true);
            }

            total += s;
        }
    }

    // handle last pattern in file
    let grid = Grid::new(&pattern);
    // grid.print_map();
    // println!();
    let mut s = check_horizontal(&grid, true);
    if s == 0 {
        s = check_vertical(&grid, true);
    }
    total += s;

    return total;
}

fn check_horizontal(grid: &Grid, check_smudge: bool) -> usize {
    let mut line_of_symmetry = 0;
    for i in 1..grid.height() {
        let a = &grid.map[i];
        let b = &grid.map[i - 1];
        if check_smudge && get_diff(&a, &b) == 1 {
            // println!("h found new line of symmetry at {}", i);
            line_of_symmetry = i;
            break;
        }
        if a == b {
            if i == 1 {
                line_of_symmetry = i;
                break;
            }
            let height = (grid.height() - i).min(i);
            let mut top: Vec<Vec<u8>> = Vec::new();
            let mut bottom: Vec<Vec<u8>> = Vec::new();
            for j in i - height..i {
                top.push(grid.map[j].clone());
            }
            for j in i..i + height {
                bottom.push(grid.map[j].clone());
            }

            bottom.reverse();
            // println!("t {:?}", top);
            // println!("b {:?}", bottom);
            if check_smudge {
                let mut diff = 0;
                for j in 0..top.len() {
                    diff += get_diff(&top[j], &bottom[j]);
                }
                if diff == 1 {
                    // println!("h found new line of symmetry at {}", i);
                    line_of_symmetry = i;
                    break;
                }
            }

            if top == bottom {
                line_of_symmetry = i;
                break;
            }
        }
    }

    return line_of_symmetry * 100;
}

fn check_vertical(grid: &Grid, check_smudge: bool) -> usize {
    let mut line_of_symmetry = 0;
    for i in 1..grid.width() {
        let col1: Vec<u8> = grid.map.iter().map(|x| x[i - 1]).collect();
        let col2: Vec<u8> = grid.map.iter().map(|x| x[i]).collect();
        if check_smudge && get_diff(&col1, &col2) == 1 {
            // println!("v found new line of symmetry at {}", i);
            line_of_symmetry = i;
            break;
        }
        if col1 == col2 {
            if i == 1 {
                line_of_symmetry = i;
                break;
            }
            let width = (grid.width() - i).min(i);
            let mut left: Vec<Vec<u8>> = Vec::new();
            let mut right: Vec<Vec<u8>> = Vec::new();
            for j in i - width..i {
                left.push(grid.map.iter().map(|x| x[j]).collect());
            }
            for j in i..i + width {
                right.push(grid.map.iter().map(|x| x[j]).collect());
            }
            right.reverse();

            if check_smudge && left != right {
                let mut diff = 0;
                for j in 0..left.len() {
                    // println!("{} L {:?}",i, left[j]);
                    // println!("{} R {:?}",i, right[j]);
                    diff += get_diff(&left[j], &right[j]);
                }
                if diff == 1 {
                    // println!("v found new line of symmetry at {}", i);
                    line_of_symmetry = i;
                    break;
                }
            }
            if left == right {
                line_of_symmetry = i;
                break;
            }
        }
    }

    return line_of_symmetry;
}

fn get_diff<T: std::cmp::PartialEq>(a: &Vec<T>, b: &Vec<T>) -> usize {
    return a
        .iter()
        .enumerate()
        .fold(0, |acc, x| if x.1 != &b[x.0] { acc + 1 } else { acc });
}
