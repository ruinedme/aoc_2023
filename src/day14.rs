#![allow(clippy::needless_return)]

use grid::Grid;
use timer::profile;

pub fn run_day14(inputs: &str) {
    profile! {
        let day14_1 = day14_1(inputs);
        println!("Day 14-1: {day14_1}");
    }

    profile! {
        let day14_2 = day14_2(inputs);
        println!("Day 14-2: {day14_2}");
    }
}

fn day14_1(inputs: &str) -> usize {
    let grid = Grid::new(inputs);
    let mut rocks = grid.find_all(&b'O');

    rocks = rocks.iter().map(|rock| {
        let mut y = rock.0;
        let mut boulders = 0;
        while y > 0{
            match grid.map[y - 1][rock.1] {
                b'O' => boulders += 1,
                b'#' => break,
                _ => (),
            }
            y -= 1;
        }
        (y+boulders,rock.1)
    }).collect();

    return rocks.iter().map(|(y,_)| grid.height() - y).sum();
}

fn day14_2(inputs: &str) -> usize {
    let mut grid = Grid::new(inputs);
    let mut sets: Vec<Vec<Vec<u8>>> = Vec::new();
    sets.push(grid.map.clone());

    let cycles = 1000;
    for _ in 0..cycles {
        let mut rocks = grid.find_all(&b'O');
        
        // tilt north
        rocks = rocks.iter().map(|rock| {
            let mut y = rock.0;
            let mut boulders = 0;
            while y > 0 {
                match grid.map[y-1][rock.1] {
                    b'O' => boulders += 1,
                    b'#' => break,
                    _=> (),
                }
                y -= 1;
            }
            grid.map[rock.0][rock.1] = b'.';
            grid.map[y+boulders][rock.1] = b'O';
            (y+boulders,rock.1)
        }).collect();
        rocks.sort();

        //tilt west
        rocks = rocks.iter().map(|rock| {
            let mut x = rock.1;
            let mut boulders = 0;
            while x > 0 {
                match grid.map[rock.0][x-1] {
                    b'O' => boulders += 1,
                    b'#' => break,
                    _=> (),
                }
                x -= 1;
            }

            grid.map[rock.0][rock.1] = b'.';
            grid.map[rock.0][x+boulders] = b'O';
            (rock.0,x+boulders)
        }).collect();
        rocks.sort();
        rocks.reverse();
        
        //tilt south
        rocks = rocks.iter().map(|rock| {
            let mut y = rock.0;
            let mut boulders = 0;
            while y < grid.height() - 1 {
                match grid.map[y+1][rock.1] {
                    b'O' => boulders += 1,
                    b'#' => break,
                    _=> (),
                }
                y += 1;
            }
            grid.map[rock.0][rock.1] = b'.';
            grid.map[y-boulders][rock.1] = b'O';
            (y-boulders,rock.1)
        }).collect();

        rocks.sort();
        rocks.reverse();
        
        //tilt east
        rocks.iter().for_each(|rock| {
            let mut x = rock.1;
            let mut boulders = 0;
            while x < grid.width() - 1 {
                match grid.map[rock.0][x+1] {
                    b'O' => boulders += 1,
                    b'#' => break,
                    _=> (),
                }
                x += 1;
            }
            
            grid.map[rock.0][rock.1] = b'.';
            grid.map[rock.0][x-boulders] = b'O';
            // (rock.0,x-boulders)
        });

        if sets.contains(&grid.map) {
            let index = sets.iter().position(|x| x == &grid.map).unwrap();
            let cycle_len = sets.len() - index;
            let final_index = index + (1_000_000_000 - index) % cycle_len;
            grid.map = sets[final_index].clone();
            let final_rocks = grid.find_all(&b'O');
            return final_rocks.iter().map(|(y,_)| grid.height() - y).sum();
        }
        sets.push(grid.map.clone());
    }

    let rocks = grid.find_all(&b'O');
    return rocks.iter().map(|(y,_)| grid.height() - y).sum();
}