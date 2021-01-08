//! `roux` provides data-structures and functions
//! to implement Cellular Automata.
//! The grid is of toroidal shape, i.e. the coordinate
//! values/neighbours wrap around. It also uses a statically
//! allocated grid to sidestep the need for dynamic memory
//! management.
#![no_std]

// tweak here for grid size / memory usage
const VERTICAL_MAX: usize = u8::MAX as usize;
const HORIZONTAL_MAX: usize = u8::MAX as usize;

/// The state of a cell.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CellState {
    Dead,
    Alive,
}

/// A structure to encode a grid with cells.
/// Cell positions start at top left corner.
pub struct Grid {
    // size allows for 256x256 cells -> enough for embedded
    // -> for more adjust the data types
    horizontal_size: u8,
    vertical_size: u8,
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
            cells: [[CellState::Dead; HORIZONTAL_MAX]; VERTICAL_MAX],
        }
    }

    /// Retrieve a cell state (for modification).
    ///
    /// # Arguments
    /// * `h`: horizontal coordinate
    /// * `v`: vertical coordinate
    pub fn get_cellstate(&self, h: u8, v: u8) -> CellState {
        if h >= self.horizontal_size {
            panic!("horizontal coordinate too large")
        }
        if v >= self.vertical_size {
            panic!("vertical coordinate too large")
        }
        return self.cells[h as usize][v as usize];
    }

    /// Retrieve a cell state (for modification) using a coordinate tuple.
    ///
    /// # Arguments
    /// * `hv`: tuple (horizontal coordinate, vertical coordinate)
    pub fn get_cellstate_hv(&self, hv: (u8, u8)) -> CellState {
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

/// A universe contains
struct Universe {
    iteration: usize,                   // counter for current iteration
    grid: Grid,                         // current grid state
    shadow: Grid,                       // temporary grid to calculate new state
    automaton: fn(u8, u8) -> CellState, // transformation function / automaton
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
        assert_eq!(c, CellState::Dead);

        // test using tuple
        c = g.get_cellstate_hv((1, 2));
        assert_eq!(c, CellState::Dead);
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
        g.set_cellstate(1, 8, CellState::Alive);
        let c = g.get_cellstate(1, 8);
        assert_eq!(c, CellState::Alive);

        // use tuple
        g.set_cellstate_hv((2, 5), CellState::Alive);
        g.get_cellstate(2, 5);
        assert_eq!(c, CellState::Alive);
    }

    #[test]
    #[should_panic]
    fn grid_set_cell_v_too_large() {
        let mut g = Grid::new(3, 17);
        g.set_cellstate(1, 17, CellState::Alive);
    }

    #[test]
    #[should_panic]
    fn grid_set_cell_h_too_large() {
        let mut g = Grid::new(3, 1);
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
}
