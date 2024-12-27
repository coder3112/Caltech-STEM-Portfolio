use rand::prelude::*;

use crate::matrix_gen::{zero_matrix_gen, Matrix};

#[derive(Clone)]
pub enum Tile {
    Empty = 0,
    Walkable = 1,
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

pub fn reset_automata(size: u16, rng: &mut StdRng, fill_percent: f64) -> Matrix {
    let mut automata_grid = zero_matrix_gen(size).unwrap();
    for x in 0..automata_grid.width {
        for y in 0..automata_grid.height {
            automata_grid.data[x as usize][y as usize] = u8::from(rng.gen_bool(fill_percent));
        }
    }
    automata_grid
}

pub fn get_neighbour_cell_count(level: &mut Matrix, x: u16, y: u16) -> u8 {
    let mut count = 0u8;
    if x > 0 {
        count += level.data[(x - 1) as usize][y as usize];
        if y > 0 {
            count += level.data[(x - 1) as usize][(y - 1) as usize];
        }
    }
    if y > 0 {
        count += level.data[(x - 1) as usize][y as usize];
        if x < level.width - 1 {
            count += level.data[(x - 1) as usize][(y - 1) as usize];
        }
    }
    if x < level.width - 1 {
        count += level.data[(x - 1) as usize][y as usize];
        if y < level.height - 1 {
            count += level.data[(x - 1) as usize][(y - 1) as usize];
        }
    }
    if y < level.height - 1 {
        count += level.data[(x - 1) as usize][y as usize];
        if x > 0 {
            count += level.data[(x - 1) as usize][(y - 1) as usize];
        }
    }
    count
}

pub fn step(level: &mut Matrix, live_cells_required: u8) -> &Matrix {
    let mut buffer = level.data.clone();
    for x in 1..level.width {
        for y in 1..level.height {
            let live_cells =
                buffer[x as usize][y as usize] + get_neighbour_cell_count(level, x, y);

            if live_cells >= live_cells_required {
                buffer[x as usize][y as usize] = 1
            } else {
                buffer[x as usize][y as usize] = 0
            }
        }
    }
    level.data = buffer;
    let level = &*level;
    level.to_owned()
}
