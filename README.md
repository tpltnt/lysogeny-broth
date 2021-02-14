Substrate for cellular automata in Rust.
Named after the [LB medium](https://en.wikipedia.org/wiki/Lysogeny_broth)
used in mircobiology. Also look into Angelina Fanny Hesse ;)
The underlying grid is of toroidal shape, i.e. the coordinate
values/neighbours wrap around. This code is dual-licensed
under the MIT/Apache 2.0 licenses.


# examples
* retrieve value as grid: `cargo run --example retrieval`
* implementation of rule 30: `cargo run --example rule30`
* save grid states as JSON: ` cargo run --example json`

# features

## dead-alive-only

Enable utility functions for binary (only) cell state.

## dead-alive-into-bool

Allow dead/alive cell to be converted into boolean values.

## dead-alive-u8-utils

Group 8 binary cell states into an octet for nicer processing and i/o.



# versions / changes

## upcoming
* documentation extended
* minor code cleaning
* `rule30` example minimised
* fixed signature of `cs8_into_u8()` to be internally compatible
* added `u8_into_cs8` utility function
* grouped cellstate / u8 conversions under feature "dead-alive-u8-utils"
  * droped feature "dead-alive-into-group-u8"
* added JSON example

## 1.1
* binary cell states are explicit feature now: "dead-alive-only"
* optional conversion of binary cell states into boolean value as feature: "dead-alive-into-bool"
* convert eight cellstates into an u8 (octet) via feature: "dead-alive-into-group-u8"

## 1.0
* grid works (setting & retrieving cell states)
* universe implemented (rules via function pointer)
* tests
* examples
* documentation


# links
* [crate documentation](https://docs.rs/lysogeny-broth/)
* [wikipedia: cellular automaton](https://en.wikipedia.org/wiki/Cellular_automaton)
* [Wolfram Atlas - various types of one-dimensional cellular automata](http://atlas.wolfram.com/TOC/TOC_200.html)
* [MathWorld: Rule 30](https://mathworld.wolfram.com/Rule30.html)
* [ca-rules](https://crates.io/crates/ca-rules) ... a possible compagnion crate
* [JSON](https://www.json.org/json-en.html)
