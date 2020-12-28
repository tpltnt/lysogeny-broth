use roux::*;

fn main() {
    println!("basic retrieval example");
    let g = roux::Grid::new(3, 1);
    let _c = g.get_cell(3, 0);
}
