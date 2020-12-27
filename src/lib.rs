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
struct Cell<'a> {
    state: CellState,
    horizontal_position: u8, // for more adjust data types
    vertical_position: u8,   // -> see also: Grid
    prev: *mut Cell<'a>,
    next: *mut Cell<'a>,
}

impl Cell<'_> {
    /// Create a new dead cell at top-left position.
    pub fn new() -> Cell<'static> {
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
struct Grid<'a> {
    // size allows for 256x256 cells -> enough for embedded
    // -> for more adjust the data types
    horizontal_size: u8,
    vertical_size: u8,
    start: *mut Cell<'a>,
}

impl Grid<'_> {
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
    pub fn new(h_size: u8, v_size: u8) -> Grid<'static> {
        let mut lastcell: *mut Cell<'_> = core::ptr::null_mut();

        // create/allocate all cells as linked list
        // (in reverse order)
        for h in 0..h_size {
            for v in 0..v_size {
                let mut c = Cell::new();
                c.horizontal_position = h;
                c.vertical_position = v;
                c.prev = lastcell;
                lastcell = &mut c as *mut Cell<'_>;
            }
        }

        // link them the other way
        loop {
            unsafe {
                // first cell allocated, prev ptr is NULL -> stop & quit
                if (*lastcell).horizontal_position != 0 && (*lastcell).vertical_position != 0 {
                    break;
                }
                (*(*lastcell).prev).next = lastcell; // the previous cell has the current cell as next one
                lastcell = (*lastcell).prev; // go back one cell & repeat
            }
        }

        // lastcell now points to first element in list
        // -> add to grid as starting point
        Grid {
            horizontal_size: h_size,
            vertical_size: v_size,
            start: lastcell,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // check default values
    fn cell_new() {
        let c = Cell::new();
        assert_eq!(c.state, CellState::Dead);
        assert_eq!(c.horizontal_position, 0);
        assert_eq!(c.vertical_position, 0);
    }

    #[test]
    // check grid creation values
    fn grid_new() {
        let g = Grid::new(5, 23);
        assert_eq!(g.horizontal_size, 5);
        assert_eq!(g.vertical_size, 23);
    }
}
