mod utils;

use fixedbitset::FixedBitSet;
use wasm_bindgen::prelude::*;
use web_sys::console;

const WIDTH: u32 = 128;
const HEIGHT: u32 = 128;

pub struct Timer<'a> {
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Self {
        console::time_with_label(name);
        Self { name }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        console::time_end_with_label(self.name);
    }
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

    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> &FixedBitSet {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells.set(idx, true);
        }
    }

    fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;

        let north = if row == 0 { self.height - 1 } else { row - 1 };
        let south = if row == self.height { 0 } else { row + 1 };
        let west = if col == 0 { self.width - 1 } else { col - 1 };
        let east = if col == self.width { 0 } else { col + 1 };

        for (r, c) in [
            (north, west),
            (north, col),
            (north, east),
            (row, east),
            (south, east),
            (south, col),
            (south, west),
            (row, west),
        ] {
            let idx = self.get_index(r, c);
            count += self.cells[idx] as u8;
        }

        count
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Self {
        utils::set_panic_hook();

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

    pub fn reset(&mut self) {
        (0..self.cells.len()).for_each(|idx| {
            self.cells.set(idx, js_sys::Math::random() >= 0.5);
        });
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

    /// Set the width of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = FixedBitSet::with_capacity((width * self.height) as usize);
    }

    /// Set the height of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = FixedBitSet::with_capacity((self.width * height) as usize);
    }

    pub fn add_glider(&mut self, row: u32, column: u32) {
        let height = self.height;
        let width = self.width;
        for delta_row in (1..=2).rev().map(|i| height - i).chain(0..=2) {
            for delta_col in (1..=2).rev().map(|i| width - i).chain(0..=2) {
                let row = (row + delta_row) % height;
                let col = (column + delta_col) % width;
                let idx = self.get_index(row, col);
                self.cells.set(idx, false);
            }
        }

        self.set_cells(&[
            ((row + height - 1) % height, (column + 1) % width),
            (row, (column + width - 1) % width),
            (row, (column + 1) % width),
            ((row + 1) % height, column),
            ((row + 1) % height, (column + 1) % width),
        ]);
    }

    pub fn add_pulsar(&mut self, row: u32, column: u32) {
        let height = self.height;
        let width = self.width;
        for delta_row in (1..=7).rev().map(|i| height - i).chain(0..=7) {
            for delta_col in (1..=7).rev().map(|i| width - i).chain(0..=7) {
                let row = (row + delta_row) % height;
                let col = (column + delta_col) % width;
                let idx = self.get_index(row, col);
                self.cells.set(idx, false);
            }
        }

        self.set_cells(&[
            ((row + height - 6) % height, (column + width - 4) % width),
            ((row + height - 6) % height, (column + width - 3) % width),
            ((row + height - 6) % height, (column + width - 2) % width),
            ((row + height - 6) % height, (column + 2) % width),
            ((row + height - 6) % height, (column + 3) % width),
            ((row + height - 6) % height, (column + 4) % width),
            ((row + height - 4) % height, (column + width - 6) % width),
            ((row + height - 4) % height, (column + width - 1) % width),
            ((row + height - 4) % height, (column + 1) % width),
            ((row + height - 4) % height, (column + 6) % width),
            ((row + height - 3) % height, (column + width - 6) % width),
            ((row + height - 3) % height, (column + width - 1) % width),
            ((row + height - 3) % height, (column + 1) % width),
            ((row + height - 3) % height, (column + 6) % width),
            ((row + height - 2) % height, (column + width - 6) % width),
            ((row + height - 2) % height, (column + width - 1) % width),
            ((row + height - 2) % height, (column + 1) % width),
            ((row + height - 2) % height, (column + 6) % width),
            ((row + height - 1) % height, (column + width - 4) % width),
            ((row + height - 1) % height, (column + width - 3) % width),
            ((row + height - 1) % height, (column + width - 2) % width),
            ((row + height - 1) % height, (column + 2) % width),
            ((row + height - 1) % height, (column + 3) % width),
            ((row + height - 1) % height, (column + 4) % width),
            //
            ((row + 1) % height, (column + width - 4) % width),
            ((row + 1) % height, (column + width - 3) % width),
            ((row + 1) % height, (column + width - 2) % width),
            ((row + 1) % height, (column + 2) % width),
            ((row + 1) % height, (column + 3) % width),
            ((row + 1) % height, (column + 4) % width),
            ((row + 2) % height, (column + width - 6) % width),
            ((row + 2) % height, (column + width - 1) % width),
            ((row + 2) % height, (column + 1) % width),
            ((row + 2) % height, (column + 6) % width),
            ((row + 3) % height, (column + width - 6) % width),
            ((row + 3) % height, (column + width - 1) % width),
            ((row + 3) % height, (column + 1) % width),
            ((row + 3) % height, (column + 6) % width),
            ((row + 4) % height, (column + width - 6) % width),
            ((row + 4) % height, (column + width - 1) % width),
            ((row + 4) % height, (column + 1) % width),
            ((row + 4) % height, (column + 6) % width),
            ((row + 6) % height, (column + width - 4) % width),
            ((row + 6) % height, (column + width - 3) % width),
            ((row + 6) % height, (column + width - 2) % width),
            ((row + 6) % height, (column + 2) % width),
            ((row + 6) % height, (column + 3) % width),
            ((row + 6) % height, (column + 4) % width),
        ]);
    }

    pub fn tick(&mut self) {
        let _timer = Timer::new("Universe::tick");

        let mut next = {
            let _timer = Timer::new("allocate next cells");
            self.cells.clone()
        };

        {
            let _timer = Timer::new("new generation");
            for row in 0..self.height {
                for col in 0..self.width {
                    let idx = self.get_index(row, col);
                    let cell_is_alive = self.cells.contains(idx);

                    let live_neighbors = self.live_neighbor_count(row, col);

                    // log!(
                    //     "cell[{}, {}] is initially {} and has {} live neighbors",
                    //     row,
                    //     col,
                    //     if cell_is_alive { "alive" } else { "dead" },
                    //     live_neighbors
                    // );

                    let cell_will_be_alive = if cell_is_alive {
                        live_neighbors == 2 || live_neighbors == 3
                    } else {
                        live_neighbors == 3
                    };

                    // log!(
                    //     "    it becomes {}",
                    //     if cell_will_be_alive { "alive" } else { "dead" }
                    // );

                    next.set(idx, cell_will_be_alive);
                }
            }
        }

        let _timer = Timer::new("free old cells");
        self.cells = next;
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells.toggle(idx);
    }
}

impl Default for Universe {
    fn default() -> Self {
        Self::new_with(|i| i % 2 == 0 || i % 7 == 0)
    }
}
