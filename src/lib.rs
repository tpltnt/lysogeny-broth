//! `roux` provides data-structures and functions
//! to implement Cellular Automata.

/// The state of a cell.
enum CellState {
    Dead,
    Alive,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
