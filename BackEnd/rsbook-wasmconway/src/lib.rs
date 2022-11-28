//! # Conway Game of Life rules
//! Any live cell with:
//! 1. fewer than 2 live neighbours dies
//! 2. 2 or 3 live neighbourse lives on to the next gen
//! 3. more than 3 live neighbours dies
//! Any dead cell with:
//! 4. exactly three live neighbours becomes a live cell
mod utils;

use wasm_bindgen::prelude::*;
use std::fmt;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>
}

#[wasm_bindgen]
impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        assert!(row < self.height);
        assert!(column < self.width);
        (row * self.width + column) as usize
    }

    fn count_live_neighbor(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (col + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    /// compute the next state of the universe
    pub fn tick(&mut self) {
        let mut next_cells: Vec<Cell> = self.cells.clone();
        // Vec::with_capacity((self.width * self.height) as usize);
        
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let current_cell = self.cells[idx];
                let live_neighbours = self.count_live_neighbor(row, col);

                let next_cell_state = match (current_cell, live_neighbours) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise
                };
                next_cells[idx] = next_cell_state;
            }
        }
        self.cells = next_cells;
    }

    pub fn new(height: u32, width: u32) -> Self {

        let cells = (0..width*height)
            .map(|i| {
                if i%2 == 0 || i%7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();
        Universe {
            width,
            height,
            cells
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, " {} ", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_universe() {
        let (height, width) = (64, 64);
        let uni = Universe::new(height, width);
        assert_eq!(uni.height, height);
        assert_eq!(uni.width, width);
        assert_eq!(uni.cells.iter().len(), (height*width) as usize);
    } 

    #[test]
    fn check_some_indexes() {
        use rand::Rng;
        
        let (height, width) = (64, 64);
        let uni = Universe::new(height, width);
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let rnd_row = rng.gen_range(0..uni.height);
            let rnd_col = rng.gen_range(0..uni.width);
            let idx = uni.get_index(rnd_row, rnd_col);
            assert!(idx < (uni.height*uni.width) as usize);
        }
    }

    #[test]
    fn can_visit_all_neighbours() {

        let (height, width) = (64, 64);
        let uni = Universe::new(height, width);
        for i in 0..uni.height {
            for j in 0..uni.width {
                uni.count_live_neighbor(i, j);
            }
        }
    }

    #[test]
    fn next_state() {
        let (height, width) = (64, 64);
        let mut uni = Universe::new(height, width);
        uni.tick();
    }

    #[test]
    fn draw_universe() {
        use std::io::{self, Write};
        let (height, width) = (16, 16);
        let mut uni = Universe::new(height, width);
        assert!(write!(io::stdout(), "\n{}", uni).is_ok());
        uni.tick();
        println!("---- ---- ---- ---- ---- ---- ---- ---- ");

        assert!(write!(io::stdout(), "\n{}", uni).is_ok());
    }
}

