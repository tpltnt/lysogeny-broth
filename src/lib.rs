//! `roux` provides data-structures and functions
//! to implement Cellular Automata.
#![no_std]

/// The state of a cell.
#[derive(Debug, PartialEq)]
enum CellState {
    Dead,
    Alive,
}

/// A cell on a grid.
/// Cell positions start at top left corner.
struct Cell {
    state: CellState,
    horizontal_position: usize,
    vertical_position: usize,
    prev: *mut Cell,
    next: *mut Cell,
}

impl Cell {
    /// Create a new dead cell at top-left position.
    pub fn new() -> Cell {
        Cell {
            state: CellState::Dead,
            horizontal_position: 0,
            vertical_position: 0,
            prev: core::ptr::null_mut(),
            next: core::ptr::null_mut(),
        }
    }
}

/// A structure to encode a grid with cells.
struct Grid {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // check default values
    fn cell_new() {
        let mut c = Cell::new();
        assert_eq!(c.state, CellState::Dead);
        assert_eq!(c.horizontal_position, 0);
        assert_eq!(c.vertical_position, 0);
    }
}
