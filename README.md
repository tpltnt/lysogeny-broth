Substrate for cellular automata in Rust.
Named after the [LB medium](https://en.wikipedia.org/wiki/Lysogeny_broth)
used in mircobiology.
The underlying grid is of toroidal shape, i.e. the coordinate
values/neighbours wrap around. This code is dual-licensed
under the MIT/Apache 2.0 licenses.


# examples
* retrieve value as grid: `cargo run --example retrieval`
* implementation of rule 30: `cargo run --example rule30`


# versions / changes

## upcoming
* documentation extended
* minor code cleaning
* `rule30` example minimised

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
* [wikipedia: cellular automaton](https://en.wikipedia.org/wiki/Cellular_automaton)
* [Wolfram Atlas - various types of one-dimensional cellular automata](http://atlas.wolfram.com/TOC/TOC_200.html)
* [MathWorld: Rule 30](https://mathworld.wolfram.com/Rule30.html)
