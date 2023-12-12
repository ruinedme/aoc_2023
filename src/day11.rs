#![allow(clippy::needless_return)]

use std::collections::HashSet;

use grid::Grid;
use timer::profile;

pub fn run_day11(inputs: &str) {
    profile! {
        let day11_1 = day11_1(&inputs);
        println!("Day 11-1: {day11_1}");
    }

    profile! {
        let day11_2 = day11_2(&inputs);
        println!("Day 11-2: {day11_2}");
    }
}

fn day11_1(inputs: &str) -> usize {
    let mut grid = Grid::new(inputs);
    let mut rows_inserted = 0;
    let mut cols_inserted = 0;
    let mut empty_cols: Vec<usize> = Vec::with_capacity(grid.width());
    let mut total = 0;

    // Get Empty Rows
    let empty_rows: Vec<usize> = grid
        .map
        .iter()
        .enumerate()
        .filter(|(_, x)| !x.contains(&b'#'))
        .map(|(i, _)| i)
        .collect();

    // Insert expansion rows
    for i in empty_rows {
        let t = vec![b'.'; grid.width()];
        grid.map.insert(i + rows_inserted, t);
        rows_inserted += 1;
    }

    // Get empty columns
    for x in 0..grid.width() {
        let mut non_empty = false;
        for row in grid.map.iter() {
            match row[x] {
                b'#' => {
                    non_empty = true;
                    break;
                }
                _ => (),
            }
        }
        if !non_empty {
            empty_cols.push(x);
        }
    }

    // Insert expansion columns
    for i in empty_cols {
        grid.insert_col(b'.', i + cols_inserted);
        cols_inserted += 1;
    }

    // Get galaxy co-ords
    let galaxies = grid.find_all(&b'#');
    
    // better way to do this?
    let mut pairs: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..galaxies.len() {
        for j in 1..galaxies.len() {
            if i != j && !pairs.contains(&(j, i)) {
                pairs.insert((i, j));
            }
        }
    }

    for (a, b) in pairs {
        total += grid.manhattan_distance(&galaxies[a], &galaxies[b]);
    }

    return total;
}

fn day11_2(inputs: &str) -> usize {
    let grid = Grid::new(inputs);

    let scalar = 1_000_000;
    let mut empty_cols: Vec<usize> = Vec::with_capacity(grid.width());
    let mut total = 0;

    // Get Empty Rows
    let empty_rows: Vec<usize> = grid
        .map
        .iter()
        .enumerate()
        .filter(|(_, x)| !x.contains(&b'#'))
        .map(|(i, _)| i)
        .collect();

    // Get empty columns
    for x in 0..grid.width() {
        let mut non_empty = false;
        for row in grid.map.iter() {
            match row[x] {
                b'#' => {
                    non_empty = true;
                    break;
                }
                _ => (),
            }
        }
        if !non_empty {
            empty_cols.push(x);
        }
    }

    // Get galaxy co-ords
    let mut galaxies = grid.find_all(&b'#');
    galaxies = galaxies
        .iter()
        .map(|(y, x)| {
            let crossed_cols: isize =
                empty_cols
                    .iter()
                    .fold(0, |acc, cx| if cx < &x { acc + 1 } else { acc });
            let crossed_rows: isize =
                empty_rows
                    .iter()
                    .fold(0, |acc, cy| if cy < &y { acc + 1 } else { acc });

            let dx = (crossed_cols * (scalar - 1)).max(0) as usize;
            let dy = (crossed_rows * (scalar - 1)).max(0) as usize;
            (*y + dy, *x + dx)
        })
        .collect();

    // better way to do this?
    let mut pairs: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..galaxies.len() {
        for j in 1..galaxies.len() {
            if i != j && !pairs.contains(&(j, i)) {
                pairs.insert((i, j));
            }
        }
    }

    for (a, b) in &pairs {
        total += grid.manhattan_distance(&galaxies[*a], &galaxies[*b]);
    }

    return total;
}
