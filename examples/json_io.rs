// JSON data export example with data provided by rule 30
// https://mathworld.wolfram.com/Rule30.html
extern crate serde;
use lysogeny_broth::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
//use serde_json::Result;
//use std::collections::Vec;

// JSON data structures
#[derive(Serialize, Deserialize, Debug)]
struct OutputData {
    note: String,
    states: Vec<Vec<Vec<String>>>,
}

// each column contains a row
fn grid_to_vec(g: &Grid) -> Vec<Vec<String>> {
    let mut data = vec![];
    for v in 0..g.get_vertical_size() {
        let mut row: Vec<String> = vec![];
        for h in 0..g.get_horizontal_size() {
            let cs = g.get_cellstate(h, v);
            let csstr = format!("{:?}", cs); // convert CellState to string
            row.push(csstr);
        }
        data.push(row);
    }
    data
}

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

fn main() {
    println!("JSON i/o example");

    // (JSON) output data
    let mut odata = OutputData {
        note: "simulating rule 30".to_string(),
        states: vec![],
    };

    // test with center cell alive
    let mut u = Universe::new(3, 1, rule30);
    u.grid.set_cellstate(1, 0, CellState::Alive);
    odata.states.push(grid_to_vec(&u.grid));

    // all cells become alive in first iteration (apply the rule)
    u.update();
    odata.states.push(grid_to_vec(&u.grid));

    // another update and all die (and stay dead)
    u.update();
    odata.states.push(grid_to_vec(&u.grid));

    u.update();
    odata.states.push(grid_to_vec(&u.grid));

    // serialize data into JSON string
    let serialized = serde_json::to_string(&odata);
    let jsonstr = match serialized {
        Ok(jstr) => jstr,
        Err(error) => {
            panic!("There was a problem creating the file: {:?}", error)
        }
    };

    // write out data
    let f = File::create("simulation.json");
    let mut f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem creating the file: {:?}", error)
        }
    };

    match f.write_all(jsonstr.as_bytes()) {
        Ok(_) => {}
        Err(error) => {
            panic!("There was a problem creating the file: {:?}", error)
        }
    }
}
