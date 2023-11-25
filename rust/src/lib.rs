mod utils;

use fixedbitset::FixedBitSet;
use wasm_bindgen::prelude::*;

const WIDTH: u32 = 64;
const HEIGHT: u32 = 64;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

impl Universe {
    fn new_with<F>(f: F) -> Self
    where
        F: FnMut(&usize) -> bool,
    {
        let width = WIDTH;
        let height = HEIGHT;

        let mut cells = FixedBitSet::with_capacity((width * height) as usize);
        (0..cells.len()).filter(f).for_each(|idx| {
            cells.set(idx, true);
        });

        Self {
            width,
            height,
            cells,
        }
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1] {
            for delta_col in [self.width - 1, 0, 1] {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Self {
        Self::new_with(|_| false)
    }

    pub fn new_default() -> Self {
        Self::default()
    }

    pub fn new_with_glider() -> Self {
        let mut universe = Self::new();
        universe.add_glider(universe.height / 2, universe.width / 2);
        universe
    }

    pub fn new_random() -> Self {
        Self::new_with(|_| js_sys::Math::random() >= 0.5)
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    pub fn add_glider(&mut self, row: u32, col: u32) {
        for delta_row in [self.height - 2, self.height - 1, 0, 1, 2] {
            for delta_col in [self.width - 2, self.width - 1, 0, 1, 2] {
                let row = (row + delta_row) % self.height;
                let col = (col + delta_col) % self.width;
                let idx = self.get_index(row, col);
                self.cells.set(idx, false);
            }
        }

        for (delta_row, delta_col) in [
            (self.height - 1, 1),
            (0, self.width - 1),
            (0, 1),
            (1, 0),
            (1, 1),
        ] {
            let row = (row + delta_row) % self.height;
            let col = (col + delta_col) % self.width;
            let idx = self.get_index(row, col);
            self.cells.set(idx, true);
        }
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell_is_alive = self.cells.contains(idx);

                let live_neighbors = self.live_neighbor_count(row, col);
                let cell_will_be_alive = if cell_is_alive {
                    live_neighbors == 2 || live_neighbors == 3
                } else {
                    live_neighbors == 3
                };

                next.set(idx, cell_will_be_alive);
            }
        }

        self.cells = next;
    }
}

impl Default for Universe {
    fn default() -> Self {
        Self::new_with(|i| i % 2 == 0 || i % 7 == 0)
    }
}
