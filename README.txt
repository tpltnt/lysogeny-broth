Substrate for cellular automata in Rust.
Named after [Wilhelm Roux](https://en.wikipedia.org/wiki/Wilhelm_Roux).
It used [buddy-alloc](https://crates.io/crates/buddy-alloc) to do
memory management on embedded (no_std) systems.
The underlying grid is of toroidal shape, i.e. the coordinate values/
neighbours wrap around.

# examples
* `cargo run --example retrieval`


# links
* [buddy-alloc](https://crates.io/crates/buddy-alloc)
