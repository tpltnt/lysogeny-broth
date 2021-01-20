// a basic demo of rule 30
// https://mathworld.wolfram.com/Rule30.html
use lysogeny_broth::*;

// just print the grid on the terminal
fn print_grid(g: &Grid) {
    for v in 0..g.get_vertical_size() {
        for h in 0..g.get_horizontal_size() {
            let cs = g.get_cellstate(h, v);
            if cs == &CellState::Alive {
                print!("o");
            } else {
                print!("x");
            }
        }
        println!("");
    }
}

fn main() {
    println!("Wolfram rule 30  example");

    // implementation of rule 30
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

    // test with center cell alive
    let mut u = Universe::new(3, 1, rule30);
    u.grid.set_cellstate(1, 0, CellState::Alive);
    print_grid(&u.grid);

    // check for correct initial state
    assert_eq!(u.grid.get_cellstate(0, 0), &CellState::Dead);
    assert_eq!(u.grid.get_cellstate(1, 0), &CellState::Alive);
    assert_eq!(u.grid.get_cellstate(2, 0), &CellState::Dead);

    // more in depth sanity checks
    assert_eq!((1, 0), u.grid.get_east_coordinate(0, 0));
    assert_eq!((2, 0), u.grid.get_east_coordinate(1, 0));
    assert_eq!((0, 0), u.grid.get_east_coordinate(2, 0));
    assert_eq!((2, 0), u.grid.get_west_coordinate(0, 0));
    assert_eq!((0, 0), u.grid.get_west_coordinate(1, 0));
    assert_eq!((1, 0), u.grid.get_west_coordinate(2, 0));

    // all cells become alive in first iteration (apply the rule)
    u.update();
    print_grid(&u.grid);

    // another update and all die (and stay dead)
    u.update();
    print_grid(&u.grid);

    u.update();
    print_grid(&u.grid);
}
