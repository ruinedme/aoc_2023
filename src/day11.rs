#![allow(clippy::needless_return)]

use std::collections::HashSet;

use grid::Grid;
use timer::profile;

pub fn run_day11(inputs: &str) {
    profile! {
        let day11_1 = day11_1(inputs);
        println!("Day 11-1: {day11_1}");
    }

    profile! {
        let day11_2 = day11_2(inputs);
        println!("Day 11-2: {day11_2}");
    }
}

fn day11_1(inputs: &str) -> usize {
    let grid = Grid::new(inputs);
    let scalar = 2;

    // Get galaxy co-ords
    let mut galaxies = grid.find_all(&b'#');
    // "expand" the universe
    galaxies = expand_galaxies(galaxies, &grid, scalar);

    return sum_distances(&galaxies, &grid);
}

fn day11_2(inputs: &str) -> usize {
    let grid = Grid::new(inputs);
    let scalar = 1_000_000;

    // Get galaxy co-ords
    let mut galaxies = grid.find_all(&b'#');
    // "expand" the universe
    galaxies = expand_galaxies(galaxies, &grid, scalar);

    return sum_distances(&galaxies, &grid);
}

fn expand_galaxies(
    galaxies: Vec<(usize, usize)>,
    grid: &Grid,
    scalar: isize,
) -> Vec<(usize, usize)> {
    let mut empty_cols: Vec<usize> = Vec::with_capacity(grid.width());
    let empty_rows: Vec<usize> = grid
        .map
        .iter()
        .enumerate()
        .filter(|(_, x)| !x.contains(&b'#'))
        .map(|(i, _)| i)
        .collect();

    for x in 0..grid.width() {
        let mut non_empty = false;
        for row in grid.map.iter() {
            if row[x] == b'#' {
                non_empty = true;
                break;
            }
        }
        if !non_empty {
            empty_cols.push(x);
        }
    }

    return galaxies
        .iter()
        .map(|(y, x)| {
            let crossed_cols: isize = empty_cols
                .iter()
                .fold(0, |acc, cx| if cx < x { acc + 1 } else { acc });
            let crossed_rows: isize = empty_rows
                .iter()
                .fold(0, |acc, cy| if cy < y { acc + 1 } else { acc });

            let dx = (crossed_cols * (scalar - 1)).max(0) as usize;
            let dy = (crossed_rows * (scalar - 1)).max(0) as usize;
            (*y + dy, *x + dx)
        })
        .collect();
}

fn sum_distances(galaxies: &Vec<(usize, usize)>, grid: &Grid) -> usize {
    let mut total = 0;

    for i in 0..galaxies.len() {
        for j in i..galaxies.len() {
            if i != j {
                total += grid.manhattan_distance(&galaxies[i], &galaxies[j]);
            }
        }
    }

    return total;
}
