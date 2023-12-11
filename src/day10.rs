#![allow(clippy::needless_return)]

use std::collections::HashSet;

use grid::Grid;
use timer::profile;

pub fn run_day10(inputs: &str) {
    profile! {
        let day10_1 = day10_1(&inputs);
        println!("Day 10-1: {day10_1}");
    }

    profile! {
        let day10_2 = day10_2(&inputs);
        println!("Day 10-2: {day10_2}");
    }
}

const NORTH_SOUTH: u8 = b'|';
const EAST_WEST: u8 = b'-';
const NORTH_EAST: u8 = b'L';
const NORTH_WEST: u8 = b'J';
const SOUTH_WEST: u8 = b'7';
const SOUTH_EAST: u8 = b'F';
const GROUND: u8 = b'.';
const START: u8 = b'S';

fn day10_1(inputs: &str) -> usize {
    
    let grid = Grid::new(inputs);
    let start = grid.index_of(&START).unwrap();
    let mut previous_pos = start;
    let mut current_pos = get_connections(&grid, &start)[0];
    let mut steps = 1;

    while current_pos != start {
        let connections = get_connections(&grid, &current_pos);
        
        let next_pos: Vec<(usize,usize)> = connections.iter().filter_map(|pos| {
            if pos != &previous_pos {
                Some(*pos)
            } else {
                None
            }
        }).collect();

        previous_pos = current_pos;
        current_pos = next_pos[0];

        steps += 1;
    }

    return steps / 2;
}

// estimated 443 -- too high
fn day10_2(inputs: &str) -> usize {
    let grid = Grid::new(inputs);
    let start = grid.index_of(&START).unwrap();
    let mut previous_pos = start;
    let mut current_pos = get_connections(&grid, &start)[0];
    let mut pipes: HashSet<(usize,usize)> = HashSet::new();

    pipes.insert(start);

    // find loop
    while current_pos != start {
        pipes.insert(current_pos);
        let connections = get_connections(&grid, &current_pos);
        
        let next_pos: Vec<(usize,usize)> = connections.iter().filter_map(|pos| {
            if pos != &previous_pos {
                Some(*pos)
            } else {
                None
            }
        }).collect();

        previous_pos = current_pos;
        current_pos = next_pos[0];

    }

    // grid.print_map();
    
    // println!("{:?}",pipes);
    let min_x = pipes.iter().min_by(|x,y| x.1.cmp(&y.1)).unwrap();
    let max_x = pipes.iter().max_by(|x,y| x.1.cmp(&y.1)).unwrap();
    let min_y = pipes.iter().min_by(|x,y| x.0.cmp(&y.0)).unwrap();
    let max_y = pipes.iter().max_by(|x,y| x.0.cmp(&y.0)).unwrap();
    let mut enclosed = 0;

    let mut intersections: HashSet<(usize,usize)> = HashSet::new();
    // iterate over every tile and check if it is inside the loop
    for (y, row) in grid.map.iter().enumerate() {
        for (x,_) in row.iter().enumerate() {
            // save on checking extraneous edge tiles since they will always be outside
            if y < min_y.0 || y > max_y.0 - 1 || x < min_x.1 || x > max_x.1 {
                continue;
            }

            // only check tiles that are not a pipe boundry
            if !pipes.contains(&(y,x)) {
                let mut pipe_intersections = 0;
                let mut vertical_boundry = false;
                let mut horizontal_boundry = false;
                // let mut is_boundry = false;
                // do ray case from top of the map up to currnent tile and do even-odd algorithm
                for i in 0..y+1 {        
                    
                    if pipes.contains(&(i,x)) {
                        let current = grid.map[i][x];          
                        match current {
                            EAST_WEST => {
                                if !horizontal_boundry {
                                    horizontal_boundry = true;
                                } else {
                                    horizontal_boundry = false
                                }
                                pipe_intersections += 1;
                            },
                            SOUTH_EAST|SOUTH_WEST|NORTH_EAST|NORTH_WEST => {
                                if !vertical_boundry && !horizontal_boundry {
                                    pipe_intersections += 1;
                                }else {
                                    vertical_boundry = false;
                                }
                                
                            },
                            NORTH_SOUTH => vertical_boundry = true,
                            _ => (),
                        }
                    }
                }
                // check if point is enclosed
                if pipe_intersections > 0 && pipe_intersections % 2 == 1 {
                    enclosed += 1;
                    intersections.insert((y,x));
                }
            }
        }
    }
    print_pipes(&grid, &pipes, &intersections);
    return enclosed;
}

fn get_connections(grid: &Grid, pos: &(usize,usize)) -> Vec<(usize,usize)> {
    let neighbors = grid.get_cardinal_neighbors(pos);
    let current = grid.map[pos.0][pos.1];
    
    return neighbors.iter().filter_map(|&(y,x)|{
        
        let a = grid.map[y][x];
        if a == GROUND {
            ()
        }

        match current {
            NORTH_SOUTH => {
                if (y < pos.0 && (a == NORTH_SOUTH || a == SOUTH_EAST || a == SOUTH_WEST || a == START)) || 
                    (y > pos.0 && (a == NORTH_SOUTH || a == NORTH_EAST || a == NORTH_WEST || a == START)) {
                    Some((y,x))
                }else {
                    None
                }
            },
            NORTH_EAST => {
                if (y < pos.0 && (a == NORTH_SOUTH || a == SOUTH_EAST || a == SOUTH_WEST || a == START)) || 
                    (x > pos.1 && (a == EAST_WEST || a == SOUTH_WEST || a == NORTH_WEST || a == START)) {
                    Some((y,x))
                } else {
                    None
                }
            },
            NORTH_WEST => {
                if (y < pos.0 && (a == NORTH_SOUTH || a == SOUTH_EAST || a == SOUTH_WEST || a == START)) || 
                    (x < pos.1 && (a == EAST_WEST || a == NORTH_EAST || a == SOUTH_EAST || a == START)) {
                    Some((y,x))
                } else {
                    None
                }
            },
            EAST_WEST => {
                if (x > pos.1 && (a == EAST_WEST || a == NORTH_WEST || a == SOUTH_WEST || a == START)) || 
                    (x < pos.1 && (a == EAST_WEST || a == NORTH_EAST || a == SOUTH_EAST || a == START)) {
                    Some((y,x))
                } else {
                    None
                }
            },
            SOUTH_EAST => {
                if (y > pos.0 && (a == NORTH_SOUTH || a == NORTH_EAST || a == NORTH_WEST || a == START)) || 
                (x > pos.1 && (a == EAST_WEST || a == SOUTH_WEST || a == NORTH_WEST || a == START)){
                    Some((y,x))
                } else {
                    None
                }
            },
            SOUTH_WEST => {
                if (y > pos.0 && (a == NORTH_SOUTH || a == NORTH_EAST || a == NORTH_WEST || a == START)) || 
                (x < pos.1 && (a == EAST_WEST || a == NORTH_EAST || a == SOUTH_EAST || a == START)){
                    Some((y,x))
                } else {
                    None
                }
            },
            START => {
                if (y < pos.0 && (a == NORTH_SOUTH || a == SOUTH_EAST || a == SOUTH_WEST)) ||
                (y > pos.0 && (a == NORTH_SOUTH || a == NORTH_EAST || a == NORTH_WEST)) ||
                (x > pos.1 && (a == EAST_WEST || a == NORTH_WEST || a == SOUTH_WEST)) ||
                (x < pos.1 && (a == EAST_WEST || a == NORTH_EAST || a == SOUTH_EAST)) {
                    Some((y,x))
                } else {
                    None
                }
            },
            _ => None,
        }
    })
    .collect();
}

#[allow(dead_code)]
fn print_pipes(grid: &Grid, pipes: &HashSet<(usize,usize)>, intersections: &HashSet<(usize,usize)>) {
    for (y,row) in grid.map.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if pipes.contains(&(y,x)) {
                match *col {
                    NORTH_EAST|NORTH_WEST|SOUTH_EAST|SOUTH_WEST => print!("#"),
                    NORTH_SOUTH => print!("|"),
                    EAST_WEST => print!("-"),
                    START => print!("S"),
                    _ => (),
                }
            } else if intersections.contains(&(y,x)){
                match *col {
                    NORTH_EAST|NORTH_WEST|EAST_WEST|NORTH_SOUTH|SOUTH_EAST|SOUTH_WEST|GROUND => print!("I"),
                    _ => (),
                }
            } else {
                print!(".");
            }
        }
        println!();
    }
}