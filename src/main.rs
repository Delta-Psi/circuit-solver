mod circuit;

use std::io;

fn main() {
    // Read the circuit from standard input.
    let stdin = io::stdin();
    let circuit = circuit::Circuit::read(&mut stdin.lock());

    // Solve the circuit.
    let solution = circuit.solve();

    // Print the results.
    for (i, vv) in solution.node_voltages.iter().enumerate() {
        println!("Node {}: {} V", i, vv);
    }
    println!();

    for(i, ii) in solution.wire_currents.iter().enumerate() {
        println!("Wire {}: {} A", i, ii);
    }
}
