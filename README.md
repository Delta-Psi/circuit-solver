# circuit-solver

An *extremely* simple circuit solver written in Rust. It supports
resistors, voltage sources and current sources, and can calculate
node voltages (relative to the first node) and wire current
intensities.

## Compiling

Simply clone this repository and run `cargo build` on the root,
then run it with `cargo run < input_file`.

## Input format

The program reads the circuit from standard input. The `test/`
directory contains some sample circuits. Nodes are 0-indexed.

* Line 1: node count, wire count.
* Line 2: blank.

The rest of the input contains the data for each wire, separated
by blank lines:
* Line 1: source node, end node, component count.
* One line for each component: component type (`v`/`i`/`r`),
	component value (respectively, voltage, intensity and
	resistance).

## License

See `LICENSE.txt`
