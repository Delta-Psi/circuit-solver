use crate::circuit::*;

use nalgebra as na;
use na::base::DMatrix;
use na::base::dimension::Dynamic;

#[derive(Debug)]
pub struct SolvedCircuit {
    pub node_voltages: Vec<f64>,
    pub wire_currents: Vec<f64>,
}

impl Circuit {
    pub fn solve(&self) -> SolvedCircuit {
        // A*X = B
        let mut a = DMatrix::<f64>::zeros_generic(
            Dynamic::new(self.node_count as usize + self.wires.len()),
            Dynamic::new(self.node_count as usize + self.wires.len()));
        let mut b = DMatrix::<f64>::zeros_generic(
            Dynamic::new(self.node_count as usize + self.wires.len()),
            Dynamic::new(1));

        // compute adjacency lists
        let mut adjacency = vec![Vec::<(usize, usize, bool)>::new(); self.node_count as usize];
        for (i, wire) in self.wires.iter().enumerate() {
            adjacency[wire.begin_node as usize].push((wire.end_node as usize, i, true));
            adjacency[wire.end_node as usize].push((wire.begin_node as usize, i, false));
        }

        // build the linear system
        a[(0, 0)] = 1.0;
        for i in 1 .. self.node_count as usize {
            for (j, w, begin) in &adjacency[i] {
                if *begin {
                    a[(i, self.node_count as usize + *w)] = 1.0;
                } else {
                    a[(i, self.node_count as usize + *w)] = -1.0;
                }
            }
        }
        
        'outer: for (i, wire) in self.wires.iter().enumerate() {
            // check if the wire contains a current source
            // FIXME: won't check if there are multiple current sources
            // in the same wire (error)
            for c in &wire.components {
                if let Component::CurrentSource(int) = *c {
                    a[(self.node_count as usize + i, self.node_count as usize + i)] = 1.0;
                    b[(self.node_count as usize + i, 0)] = int;

                    continue 'outer;
                }
            }

            a[(self.node_count as usize + i, wire.begin_node as usize)] = 1.0;
            a[(self.node_count as usize + i, wire.end_node as usize)] = -1.0;

            for c in &wire.components {
                match *c {
                    Component::VoltageSource(vol) =>
                        b[(self.node_count as usize + i, 0)] -= vol,
                    Component::Resistor(res) =>
                        a[(self.node_count as usize + i, self.node_count as usize + i)] -= res,
                    _ => unreachable!(),
                }
            }
        }

        // solve the linear system
        let solution = a.qr().solve(&b).unwrap();

        SolvedCircuit {
            node_voltages: (0 .. self.node_count as usize)
                .map(|i| solution[(i, 0)])
                .collect(),
            wire_currents: (0 .. self.wires.len())
                .map(|i| solution[(self.node_count as usize + i, 0)])
                .collect(),
        }
    }
}
