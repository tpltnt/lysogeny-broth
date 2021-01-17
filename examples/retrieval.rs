// a basic demo for retrieving cell states
use roux::*;

fn main() {
    println!("basic retrieval example");
    let g = Grid::new(3, 1);
    let _c = g.get_cellstate(3, 0);
}
