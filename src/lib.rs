//! `lysogeny-broth` provides data-structures and functions
//! to implement Cellular Automata.
//! The grid is of toroidal shape, i.e. the coordinate
//! values/neighbours wrap around. It also uses a statically
//! allocated grid to sidestep the need for dynamic memory
//! management.
//! This code is dual-licensed under the MIT and Apache 2.0 licenses.
/*
Copyright (c) 2021 tpltnt

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.


Copyright 2021 tpltnt

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
 */
#![no_std]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

/// Tweak here for vertical grid size / memory usage.
/// `u8` was chosen to accommodate memory constraints.
const VERTICAL_MAX: usize = u8::MAX as usize;
/// Tweak here for horizontal grid size / memory usage.
/// `u8` was chosen to accommodate memory constraints.
const HORIZONTAL_MAX: usize = u8::MAX as usize;

/// The state of a cell.
///
/// # Remarks
/// A cell has no concept of its neighbours. Everything
/// in terms of space is handled by the Grid.
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg(not(feature = "dead-alive-only"))]
pub enum CellState {
    Dummy,
}

/// The state of a cell. In this case it is either
/// dead or alive.
///
/// # Remarks
/// A cell has no concept of its neighbours. Everything
/// in terms of space is handled by the Grid.
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg(feature = "dead-alive-only")]
pub enum CellState {
    /// represents a dead cell
    Dead,
    /// represents a living cell
    Alive,
}

impl CellState {
    #[cfg(feature = "dead-alive-into-bool")]
    /// If cells can be either alive or dead, then
    /// their state could be converted into a boolean
    /// value where true means 'alive' (i.e. the cell
    /// is there) and false means 'dead') (i.e. no cell).
    pub fn into_bool(self) -> bool {
        if self == CellState::Dead {
            return false;
        }
        true
    }
}

#[cfg(feature = "dead-alive-into-group-u8")]
/// Convert eight binary cell states into a u8 / octet.
/// A dead cell becomes a 0, an alive one a 1.
pub fn cs8_into_u8(cs: [CellState; 8]) -> u8 {
    let mut rdata: u8 = 0b00000000;
    for s in cs.iter() {
        rdata = rdata.rotate_left(1);
        if s == &CellState::Alive {
            // set bit
            let bit: u8 = 0b00000001;
            rdata = rdata | bit;
        }
    }
    return rdata;
}

/// A structure to encode a grid with cells.
/// Cell positions start at the top left corner.
/// The grid handles everything in terms of space.
/// The retrieval methods for different neighbours
/// allow for the implementation of different
/// neighbourhoods (e.g. von Neumann, Moore ...) or
/// even arbitrary functions to determine the new
/// value of a cell based on its adjacent cells
/// (or even state of the whole grid).
#[derive(Copy, Clone, Debug)]
pub struct Grid {
    /// Allow size allows for 256 horizontal cells.
    /// This is good enough for embedded environments.
    /// If you need for more adjust the data types as needed.
    horizontal_size: u8,
    /// Allow size allows for 256 vertical cells.
    /// This is good enough for embedded environments.
    /// If you need for more adjust the data types as needed.
    vertical_size: u8,
    /// The actual arrays to hold cell states.
    cells: [[CellState; HORIZONTAL_MAX]; VERTICAL_MAX],
}

impl Grid {
    /// Create a new grid with the given dimensions and
    /// fill it with default (dead) cells.
    ///
    /// # Arguments
    /// * `h_size`: horizontal dimension/size as number of cells
    /// * `v_size`: vertical dimension/size as number of cells
    ///
    /// # Remarks
    ///
    /// `u8` was chosen to stay below `usize::MAX` for a `u8` x `u8`
    /// grid. 256x256 are currently enough cells for embedded applications.
    /// Larger grid sizes have to keep the target usize (thus architecture)
    /// in mind and can be adjusted appropriately.
    pub fn new(h_size: u8, v_size: u8) -> Grid {
        if h_size == 0 {
            panic!("horizontal coordinate too small")
        }
        if v_size == 0 {
            panic!("vertical coordinate too small")
        }
        if h_size as usize > HORIZONTAL_MAX {
            panic!("horizontal coordinate too large")
        }
        if v_size as usize > VERTICAL_MAX {
            panic!("vertical coordinate too large")
        }

        Grid {
            horizontal_size: h_size,
            vertical_size: v_size,
            #[cfg(not(feature = "dead-alive-only"))]
            cells: [[CellState::Dummy; HORIZONTAL_MAX]; VERTICAL_MAX],
            #[cfg(feature = "dead-alive-only")]
            cells: [[CellState::Dead; HORIZONTAL_MAX]; VERTICAL_MAX],
        }
    }

    /// Get the number of columns (i.e. horizontal size)
    pub fn get_horizontal_size(&self) -> u8 {
        self.horizontal_size
    }

    /// Get number of rows (i.e. vertical size)
    pub fn get_vertical_size(&self) -> u8 {
        self.vertical_size
    }

    /// Retrieve a cell state (for modification).
    ///
    /// # Arguments
    /// * `h`: horizontal coordinate
    /// * `v`: vertical coordinate
    pub fn get_cellstate(&self, h: u8, v: u8) -> &CellState {
        if h >= self.horizontal_size {
            panic!("horizontal coordinate too large")
        }
        if v >= self.vertical_size {
            panic!("vertical coordinate too large")
        }
        &self.cells[h as usize][v as usize]
    }

    /// Retrieve a cell state (for modification) using a coordinate tuple.
    ///
    /// # Arguments
    /// * `hv`: tuple (horizontal coordinate, vertical coordinate)
    pub fn get_cellstate_hv(&self, hv: (u8, u8)) -> &CellState {
        self.get_cellstate(hv.0, hv.1)
    }

    /// Set a (modified) cell state.
    ///
    /// # Arguments
    /// * `h`: horizontal coordinate
    /// * `v`: vertical coordinate
    pub fn set_cellstate(&mut self, h: u8, v: u8, state: CellState) {
        if h >= self.horizontal_size {
            panic!("horizontal coordinate too large")
        }
        if v >= self.vertical_size {
            panic!("vertical coordinate too large")
        }
        self.cells[h as usize][v as usize] = state;
    }

    /// Set a (modified) cell state using a coordination tuple.
    ///
    /// # Arguments
    /// * `hv`: tuple (horizontal coordinate, vertical coordinate)
    pub fn set_cellstate_hv(&mut self, hv: (u8, u8), state: CellState) {
        self.set_cellstate(hv.0, hv.1, state)
    }

    /// Get coordinates of "northern" cell relative
    /// to the given grid coordinates.
    ///
    /// # Arguments
    /// * `h`: horizontal coordinate
    /// * `v`: vertical coordinate
    pub fn get_north_coordinate(&self, h: u8, v: u8) -> (u8, u8) {
        if h >= self.horizontal_size {
            panic!("horizontal coordinate too large")
        }
        if v >= self.vertical_size {
            panic!("vertical coordinate too large")
        }
        if v == 0 {
            return (h, self.vertical_size - 1);
        }
        (h, v - 1)
    }

    /// Get coordinates of "northern" cell relative
    /// to the given grid coordinates.
    ///
    /// # Arguments
    /// * `hv`: tuple (horizontal coordinate, vertical coordinate)
    pub fn get_north_coordinate_hv(&self, hv: (u8, u8)) -> (u8, u8) {
        self.get_north_coordinate(hv.0, hv.1)
    }

    /// Get coordinates of "eastern" cell relative
    /// to the given grid coordinates.
    ///
    /// # Arguments
    /// * `h`: horizontal coordinate
    /// * `v`: vertical coordinate
    pub fn get_east_coordinate(&self, h: u8, v: u8) -> (u8, u8) {
        if h >= self.horizontal_size {
            panic!("horizontal coordinate too large")
        }
        if v >= self.vertical_size {
            panic!("vertical coordinate too large")
        }
        if h == self.horizontal_size - 1 {
            return (0, v);
        }
        (h + 1, v)
    }

    /// Get coordinates of "eastern" cell relative
    /// to the given grid coordinates.
    ///
    /// # Arguments
    /// * `hv`: tuple (horizontal coordinate, vertical coordinate)
    pub fn get_east_coordinate_hv(&self, hv: (u8, u8)) -> (u8, u8) {
        self.get_east_coordinate(hv.0, hv.1)
    }

    /// Get coordinates of "southern" cell relative
    /// to the given grid coordinates.
    ///
    /// # Arguments
    /// * `h`: horizontal coordinate
    /// * `v`: vertical coordinate
    pub fn get_south_coordinate(&self, h: u8, v: u8) -> (u8, u8) {
        if h >= self.horizontal_size {
            panic!("horizontal coordinate too large")
        }
        if v >= self.vertical_size {
            panic!("vertical coordinate too large")
        }
        if v == self.vertical_size - 1 {
            return (h, 0);
        }
        (h, v + 1)
    }

    /// Get coordinates of "eastern" cell relative
    /// to the given grid coordinates.
    ///
    /// # Arguments
    /// * `hv`: tuple (horizontal coordinate, vertical coordinate)
    pub fn get_south_coordinate_hv(&self, hv: (u8, u8)) -> (u8, u8) {
        self.get_south_coordinate(hv.0, hv.1)
    }

    /// Get coordinates of "western" cell relative
    /// to the given grid coordinates.
    ///
    /// # Arguments
    /// * `h`: horizontal coordinate
    /// * `v`: vertical coordinate
    pub fn get_west_coordinate(&self, h: u8, v: u8) -> (u8, u8) {
        if h >= self.horizontal_size {
            panic!("horizontal coordinate too large")
        }
        if v >= self.vertical_size {
            panic!("vertical coordinate too large")
        }
        if h == 0 {
            return (self.horizontal_size - 1, v);
        }
        (h - 1, v)
    }

    /// Get coordinates of "western" cell relative
    /// to the given grid coordinates.
    ///
    /// # Arguments
    /// * `hv`: tuple (horizontal coordinate, vertical coordinate)
    pub fn get_west_coordinate_hv(&self, hv: (u8, u8)) -> (u8, u8) {
        self.get_west_coordinate(hv.0, hv.1)
    }

    /// Get coordinates of "north eastern" cell relative
    /// to the given grid coordinates.
    ///
    /// # Arguments
    /// * `h`: horizontal coordinate
    /// * `v`: vertical coordinate
    pub fn get_northeast_coordinate(&self, h: u8, v: u8) -> (u8, u8) {
        self.get_north_coordinate_hv(self.get_east_coordinate(h, v))
    }

    /// Get coordinates of "north eastern" cell relative
    /// to the given grid coordinates.
    ///
    /// # Arguments
    /// * `hv`: tuple (horizontal coordinate, vertical coordinate)
    pub fn get_northeast_coordinate_hv(&self, hv: (u8, u8)) -> (u8, u8) {
        self.get_north_coordinate_hv(self.get_east_coordinate(hv.0, hv.1))
    }

    /// Get coordinates of "south eastern" cell relative
    /// to the given grid coordinates.
    ///
    /// # Arguments
    /// * `h`: horizontal coordinate
    /// * `v`: vertical coordinate
    pub fn get_southeast_coordinate(&self, h: u8, v: u8) -> (u8, u8) {
        self.get_south_coordinate_hv(self.get_east_coordinate(h, v))
    }

    /// Get coordinates of "south eastern" cell relative
    /// to the given grid coordinates.
    ///
    /// # Arguments
    /// * `hv`: tuple (horizontal coordinate, vertical coordinate)
    pub fn get_southeast_coordinate_hv(&self, hv: (u8, u8)) -> (u8, u8) {
        self.get_south_coordinate_hv(self.get_east_coordinate(hv.0, hv.1))
    }

    /// Get coordinates of "south western" cell relative
    /// to the given grid coordinates.
    ///
    /// # Arguments
    /// * `h`: horizontal coordinate
    /// * `v`: vertical coordinate
    pub fn get_southwest_coordinate(&self, h: u8, v: u8) -> (u8, u8) {
        self.get_south_coordinate_hv(self.get_west_coordinate(h, v))
    }

    /// Get coordinates of "south western" cell relative
    /// to the given grid coordinates.
    ///
    /// # Arguments
    /// * `hv`: tuple (horizontal coordinate, vertical coordinate)
    pub fn get_southwest_coordinate_hv(&self, hv: (u8, u8)) -> (u8, u8) {
        self.get_south_coordinate_hv(self.get_west_coordinate(hv.0, hv.1))
    }

    /// Get coordinates of "north western" cell relative
    /// to the given grid coordinates.
    ///
    /// # Arguments
    /// * `h`: horizontal coordinate
    /// * `v`: vertical coordinate
    pub fn get_northwest_coordinate(&self, h: u8, v: u8) -> (u8, u8) {
        self.get_north_coordinate_hv(self.get_west_coordinate(h, v))
    }

    /// Get coordinates of "north western" cell relative
    /// to the given grid coordinates.
    ///
    /// # Arguments
    /// * `hv`: tuple (horizontal coordinate, vertical coordinate)
    pub fn get_northwest_coordinate_hv(&self, hv: (u8, u8)) -> (u8, u8) {
        self.get_north_coordinate_hv(self.get_west_coordinate(hv.0, hv.1))
    }
}

/// A universe contains everything you need to enable
/// Cellular Automata to do their thing.
#[derive(Copy, Clone)]
pub struct Universe {
    /// The current state of the grid.
    pub grid: Grid,
    /// Temporary internal grid to calculate new state.
    shadow: Grid,
    /// The transformation function / cellular automaton.
    automaton: fn(u8, u8, &Grid) -> CellState,
}

impl Universe {
    /// Create a new universe with only dead cells.
    ///
    /// # Arguments
    /// * `h_size`: horizontal dimension/size as number of cells
    /// * `v_size`: vertical dimension/size as number of cells
    /// * `rules`: a function mapping a coordinate (and thus the state of a cell) on a grid to a new state
    pub fn new(h_size: u8, v_size: u8, rules: fn(u8, u8, &Grid) -> CellState) -> Universe {
        Universe {
            grid: Grid::new(h_size, v_size),
            shadow: Grid::new(h_size, v_size),
            automaton: rules,
        }
    }

    /// Update the universe according to the given state and rules
    pub fn update(&mut self) {
        // calculate new state from original grid and
        // (temporarily) save in shadow grid
        for h in 0..self.grid.horizontal_size {
            for v in 0..self.grid.vertical_size {
                let state = (self.automaton)(h, v, &self.grid);
                self.shadow.set_cellstate(h, v, state);
            }
        }

        // copy over new (shadow) state to public grid
        //self.grid = self.shadow;
        for h in 0..self.grid.horizontal_size {
            for v in 0..self.grid.vertical_size {
                let state = self.shadow.get_cellstate(h, v);
                self.grid.set_cellstate(h, v, *state); // does not work
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // check grid creation values
    fn grid_new() {
        let g = Grid::new(5, 23);
        assert_eq!(g.horizontal_size, 5);
        assert_eq!(g.vertical_size, 23);
    }

    #[test]
    #[should_panic]
    fn grid_new_too_small() {
        let _ = Grid::new(0, 1);
        let _ = Grid::new(1, 0);
    }

    #[test]
    // check grid creation values
    fn grid_get_cellstate() {
        let g = Grid::new(3, 17);
        let mut c = g.get_cellstate(1, 8);
        #[cfg(not(feature = "dead-alive-only"))]
        assert_eq!(c, &CellState::Dummy);
        #[cfg(feature = "dead-alive-only")]
        assert_eq!(c, &CellState::Dead);

        // test using tuple
        c = g.get_cellstate_hv((1, 2));
        #[cfg(not(feature = "dead-alive-only"))]
        assert_eq!(c, &CellState::Dummy);
        #[cfg(feature = "dead-alive-only")]
        assert_eq!(c, &CellState::Dead);
    }

    #[test]
    #[should_panic]
    fn grid_get_cell_v_too_large() {
        let g = Grid::new(3, 17);
        let _c = g.get_cellstate(1, 17);
    }

    #[test]
    #[should_panic]
    fn grid_get_cell_h_too_large() {
        let g = Grid::new(3, 1);
        let _c = g.get_cellstate(3, 0);
    }

    #[test]
    // check grid creation values
    fn grid_set_cellstate() {
        let mut g = Grid::new(3, 17);
        #[cfg(feature = "dead-alive-only")]
        g.set_cellstate(1, 8, CellState::Alive);
        let mut c = g.get_cellstate(1, 8);
        #[cfg(feature = "dead-alive-only")]
        assert_eq!(c, &CellState::Alive);

        // use tuple
        #[cfg(feature = "dead-alive-only")]
        g.set_cellstate_hv((2, 5), CellState::Alive);
        c = g.get_cellstate(2, 5);
        #[cfg(feature = "dead-alive-only")]
        assert_eq!(c, &CellState::Alive);
    }

    #[test]
    #[should_panic]
    fn grid_set_cell_v_too_large() {
        let mut g = Grid::new(3, 17);
        #[cfg(not(feature = "dead-alive-only"))]
        g.set_cellstate(1, 17, CellState::Dummy);
        #[cfg(feature = "dead-alive-only")]
        g.set_cellstate(1, 17, CellState::Alive);
    }

    #[test]
    #[should_panic]
    fn grid_set_cell_h_too_large() {
        let mut g = Grid::new(3, 1);
        #[cfg(not(feature = "dead-alive-only"))]
        g.set_cellstate(3, 0, CellState::Dummy);
        #[cfg(feature = "dead-alive-only")]
        g.set_cellstate(3, 0, CellState::Alive);
    }

    #[test]
    fn grid_get_north_coordinate() {
        let g = Grid::new(3, 4);
        let mut result = g.get_north_coordinate(1, 2);
        assert_eq!(result.0, 1);
        assert_eq!(result.1, 1);

        result = g.get_north_coordinate(2, 0);
        assert_eq!(result.0, 2);
        assert_eq!(result.1, 3);
    }

    #[test]
    #[should_panic]
    fn grid_get_north_coordinate_v_too_large() {
        let g = Grid::new(1, 4);
        let _ = g.get_north_coordinate(0, 4);
    }

    #[test]
    #[should_panic]
    fn grid_get_north_coordinate_h_too_large() {
        let g = Grid::new(1, 4);
        let _ = g.get_north_coordinate(1, 2);
    }

    #[test]
    fn grid_get_south_coordinate() {
        let g = Grid::new(3, 4);
        let mut result = g.get_south_coordinate(1, 2);
        assert_eq!(result.0, 1);
        assert_eq!(result.1, 3);

        result = g.get_south_coordinate(2, 0);
        assert_eq!(result.0, 2);
        assert_eq!(result.1, 1);
    }

    #[test]
    #[should_panic]
    fn grid_get_south_coordinate_v_too_large() {
        let g = Grid::new(1, 4);
        let _ = g.get_south_coordinate(0, 4);
    }

    #[test]
    #[should_panic]
    fn grid_get_south_coordinate_h_too_large() {
        let g = Grid::new(1, 4);
        let _ = g.get_south_coordinate(1, 2);
    }

    #[test]
    fn grid_get_west_coordinate() {
        let g = Grid::new(3, 4);
        let mut result = g.get_west_coordinate(1, 2);
        assert_eq!(result.0, 0);
        assert_eq!(result.1, 2);

        result = g.get_west_coordinate(0, 2);
        assert_eq!(result.0, 2);
        assert_eq!(result.1, 2);
    }

    #[test]
    #[should_panic]
    fn grid_get_west_coordinate_v_too_large() {
        let g = Grid::new(1, 4);
        let _ = g.get_west_coordinate(0, 4);
    }

    #[test]
    #[should_panic]
    fn grid_get_west_coordinate_h_too_large() {
        let g = Grid::new(1, 4);
        let _ = g.get_west_coordinate(1, 2);
    }

    #[test]
    fn grid_get_northeast_coordinate() {
        let g = Grid::new(3, 4);
        let mut result = g.get_northeast_coordinate(1, 2);
        assert_eq!(result.0, 2);
        assert_eq!(result.1, 1);

        result = g.get_northeast_coordinate(2, 0);
        assert_eq!(result.0, 0);
        assert_eq!(result.1, 3);
    }

    #[test]
    #[should_panic]
    fn grid_get_northeast_coordinate_v_too_large() {
        let g = Grid::new(1, 4);
        let _ = g.get_northeast_coordinate(0, 4);
    }

    #[test]
    #[should_panic]
    fn grid_get_northeast_coordinate_h_too_large() {
        let g = Grid::new(1, 4);
        let _ = g.get_northeast_coordinate(1, 2);
    }

    #[test]
    fn grid_get_southeast_coordinate() {
        let g = Grid::new(3, 4);
        let mut result = g.get_southeast_coordinate(1, 2);
        assert_eq!(result.0, 2);
        assert_eq!(result.1, 3);

        result = g.get_southeast_coordinate(2, 0);
        assert_eq!(result.0, 0);
        assert_eq!(result.1, 1);
    }

    #[test]
    #[should_panic]
    fn grid_get_southeast_coordinate_v_too_large() {
        let g = Grid::new(1, 4);
        let _ = g.get_southeast_coordinate(0, 4);
    }

    #[test]
    #[should_panic]
    fn grid_get_southeast_coordinate_h_too_large() {
        let g = Grid::new(1, 4);
        let _ = g.get_southeast_coordinate(1, 2);
    }

    #[test]
    fn grid_get_southwest_coordinate() {
        let g = Grid::new(3, 4);
        let mut result = g.get_southwest_coordinate(1, 2);
        assert_eq!(result.0, 0);
        assert_eq!(result.1, 3);

        result = g.get_southwest_coordinate(0, 0);
        assert_eq!(result.0, 2);
        assert_eq!(result.1, 1);
    }

    #[test]
    #[should_panic]
    fn grid_get_southwest_coordinate_v_too_large() {
        let g = Grid::new(1, 4);
        let _ = g.get_southwest_coordinate(0, 4);
    }

    #[test]
    #[should_panic]
    fn grid_get_southwest_coordinate_h_too_large() {
        let g = Grid::new(1, 4);
        let _ = g.get_southwest_coordinate(1, 2);
    }

    #[test]
    fn grid_get_northwest_coordinate() {
        let g = Grid::new(3, 4);
        let mut result = g.get_northwest_coordinate(1, 2);
        assert_eq!(result.0, 0);
        assert_eq!(result.1, 1);

        result = g.get_northwest_coordinate(0, 0);
        assert_eq!(result.0, 2);
        assert_eq!(result.1, 3);
    }

    #[test]
    #[should_panic]
    fn grid_get_northwest_coordinate_v_too_large() {
        let g = Grid::new(1, 4);
        let _ = g.get_northwest_coordinate(0, 4);
    }

    #[test]
    #[should_panic]
    fn grid_get_northwest_coordinate_h_too_large() {
        let g = Grid::new(1, 4);
        let _ = g.get_northwest_coordinate(1, 2);
    }

    #[test]
    fn universe_update_on_grid() {
        fn identity(h: u8, v: u8, g: &Grid) -> CellState {
            *g.get_cellstate(h, v)
        }
        let mut u1 = Universe::new(4, 6, identity);
        u1.update();
        for h in 0..4u8 {
            for v in 0..6u8 {
                let cs = u1.grid.get_cellstate(h, v);
                #[cfg(not(feature = "dead-alive-only"))]
                assert_eq!(cs, &CellState::Dummy);
                #[cfg(feature = "dead-alive-only")]
                assert_eq!(cs, &CellState::Dead);
            }
        }

        fn inversion(h: u8, v: u8, g: &Grid) -> CellState {
            match g.get_cellstate(h, v) {
                &CellState::Alive => CellState::Dead,
                &CellState::Dead => CellState::Alive,
            }
        }

        let mut u2 = Universe::new(4, 6, inversion);
        u2.update();
        for h in 0..4u8 {
            for v in 0..6u8 {
                let cs = u2.grid.get_cellstate(h, v);
                assert_eq!(cs, &CellState::Alive);
            }
        }
    }

    #[test]
    #[cfg(feature = "dead-alive-only")]
    fn universe_automaton() {
        fn inversion(h: u8, v: u8, g: &Grid) -> CellState {
            match g.get_cellstate(h, v) {
                &CellState::Alive => CellState::Dead,
                &CellState::Dead => CellState::Alive,
            }
        }

        let u = Universe::new(1, 1, inversion);
        assert_eq!(u.grid.get_cellstate(0, 0), &CellState::Dead);

        let state = (u.automaton)(0, 0, &u.grid);
        assert_eq!(state, CellState::Alive);
    }

    #[test]
    #[cfg(feature = "dead-alive-only")]
    fn universe_update_one_cell_inversion() {
        fn inversion(h: u8, v: u8, g: &Grid) -> CellState {
            match g.get_cellstate(h, v) {
                &CellState::Alive => CellState::Dead,
                &CellState::Dead => CellState::Alive,
            }
        }

        let mut u = Universe::new(1, 1, inversion);
        assert_eq!(u.grid.get_cellstate(0, 0), &CellState::Dead);

        // do it manually
        u.grid.set_cellstate(0, 0, CellState::Alive);
        assert_eq!(u.grid.get_cellstate(0, 0), &CellState::Alive);

        // reset via inversion rule
        u.update(); // TODO: -> calling update seems to fail moditying the states
                    //assert_eq!(u.shadow.get_cellstate(0, 0), &CellState::Alive); // sanity check on shadow state -> fails
                    //u.grid.set_cellstate(0,0,CellState::Dead);  // this works
        assert_eq!(u.grid.get_cellstate(0, 0), &CellState::Dead); // this fails
    }

    // test based on Wolfram rule 30
    // https://mathworld.wolfram.com/Rule30.html
    // https://en.wikipedia.org/wiki/Rule_30
    #[test]
    #[cfg(feature = "dead-alive-only")]
    fn universe_update_rule30() {
        fn rule30(h: u8, v: u8, g: &Grid) -> CellState {
            let left = g.get_west_coordinate(h, v);
            let right = g.get_east_coordinate(h, v);
            let state = (
                g.get_cellstate_hv(left),
                g.get_cellstate(h, v),
                g.get_cellstate_hv(right),
            );
            return match state {
                (CellState::Alive, CellState::Alive, CellState::Alive) => CellState::Dead,
                (CellState::Alive, CellState::Alive, CellState::Dead) => CellState::Dead,
                (CellState::Alive, CellState::Dead, CellState::Alive) => CellState::Dead,
                (CellState::Alive, CellState::Dead, CellState::Dead) => CellState::Alive,
                (CellState::Dead, CellState::Alive, CellState::Alive) => CellState::Alive,
                (CellState::Dead, CellState::Alive, CellState::Dead) => CellState::Alive,
                (CellState::Dead, CellState::Dead, CellState::Alive) => CellState::Alive,
                (CellState::Dead, CellState::Dead, CellState::Dead) => CellState::Dead,
            };
        }

        // test on dead universe -> should stay dead
        let mut u1 = Universe::new(3, 1, rule30);
        u1.update();
        for h in 0..2u8 {
            let cs = u1.grid.get_cellstate(h, 0);
            assert_eq!(cs, &CellState::Dead)
        }

        // test with center cell alive
        let mut u2 = Universe::new(3, 1, rule30);
        u2.grid.set_cellstate(1, 0, CellState::Alive);
        // check for correct initial state
        assert_eq!(u2.grid.get_cellstate(0, 0), &CellState::Dead);
        assert_eq!(u2.grid.get_cellstate(1, 0), &CellState::Alive);
        assert_eq!(u2.grid.get_cellstate(2, 0), &CellState::Dead);

        // more in depth sanity checks
        assert_eq!((1, 0), u2.grid.get_east_coordinate(0, 0));
        assert_eq!((2, 0), u2.grid.get_east_coordinate(1, 0));
        assert_eq!((0, 0), u2.grid.get_east_coordinate(2, 0));
        assert_eq!((2, 0), u2.grid.get_west_coordinate(0, 0));
        assert_eq!((0, 0), u2.grid.get_west_coordinate(1, 0));
        assert_eq!((1, 0), u2.grid.get_west_coordinate(2, 0));

        // test the rule itself
        assert_eq!(CellState::Alive, rule30(0, 0, &u2.grid));
        assert_eq!(CellState::Alive, rule30(1, 0, &u2.grid));
        assert_eq!(CellState::Alive, rule30(2, 0, &u2.grid));

        // all cells become alive in first iteration (apply the rule)
        u2.update();

        // test shadow state
        assert_eq!(u2.shadow.get_cellstate(0, 0), &CellState::Alive);
        assert_eq!(u2.shadow.get_cellstate(1, 0), &CellState::Alive);
        assert_eq!(u2.shadow.get_cellstate(2, 0), &CellState::Alive);

        // test public state
        assert_eq!(u2.grid.get_cellstate(0, 0), &CellState::Alive);
        assert_eq!(u2.grid.get_cellstate(1, 0), &CellState::Alive);
        assert_eq!(u2.grid.get_cellstate(2, 0), &CellState::Alive);

        // this universe should die on second iteration
        u2.update();
        assert_eq!(u2.grid.get_cellstate(0, 0), &CellState::Dead);
        assert_eq!(u2.grid.get_cellstate(1, 0), &CellState::Dead);
        assert_eq!(u2.grid.get_cellstate(2, 0), &CellState::Dead);
    }

    #[test]
    #[cfg(feature = "dead-alive-into-bool")]
    fn cellstate_into_bool() {
        let mut cs = CellState::Dead;
        assert_eq!(cs.into_bool(), false);
        cs = CellState::Alive;
        assert_eq!(cs.into_bool(), true);
    }

    #[test]
    #[cfg(feature = "dead-alive-into-group-u8")]
    fn util_cs8_into_u8() {
        let mut group = [CellState::Dead; 8];
        let mut result = cs8_into_u8(group);
        assert_eq!(result, 0b00000000);

        group[7] = CellState::Alive;
        result = cs8_into_u8(group);
        assert_eq!(result, 0b00000001);

        group[0] = CellState::Alive;
        result = cs8_into_u8(group);
        assert_eq!(result, 0b10000001);

        group[0] = CellState::Alive;
        group[3] = CellState::Alive;
        group[7] = CellState::Dead;
        result = cs8_into_u8(group);
        assert_eq!(result, 0b10010000);
    }
}
