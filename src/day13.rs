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
            let mut s = check_vertial(&grid);
            if s == 0 {
                s = check_horizontal(&grid);
            }

            total += s;
        }
    }

    // handle last pattern in file
    let grid = Grid::new(&pattern);
    let mut s = check_vertial(&grid);
    if s == 0 {
        s = check_horizontal(&grid);
    }
    total += s;

    return total;
}

fn day13_2(_inputs: &str) -> usize {
    return 0;
}

fn check_horizontal(grid: &Grid) -> usize {
    let mut line_of_symmetry = 0;
    for i in 1..grid.height() {
        if grid.map[i] == grid.map[i - 1] {
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
            if top == bottom {
                line_of_symmetry = i;
                break;
            }
        }
    }

    return line_of_symmetry * 100;
}

fn check_vertial(grid: &Grid) -> usize {
    let mut line_of_symmetry = 0;
    for i in 1..grid.width() {
        let col1: Vec<u8> = grid.map.iter().map(|x| x[i - 1]).collect();
        let col2: Vec<u8> = grid.map.iter().map(|x| x[i]).collect();
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
            if left == right {
                line_of_symmetry = i;
                break;
            }
        }
    }

    return line_of_symmetry;
}
