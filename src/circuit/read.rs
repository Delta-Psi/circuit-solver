use crate::circuit::*;

use std::io::BufRead;
use scan_rules::{let_scan, scan, scan_rules_impl};

impl Circuit {
    pub fn read<R: BufRead>(input: &mut R) -> Circuit {
        let mut lines = input.lines();

        // Read the node and wire counts.
        let_scan!(&lines.next().unwrap().unwrap(); (
                let node_count: u32, let wire_count: u32
        ));

        // Read the endpoints and components of each wire.
        let mut wires = Vec::<Wire>::with_capacity(wire_count as usize);
        for _ in 0 .. wire_count {
            lines.next().unwrap().unwrap();

            let_scan!(&lines.next().unwrap().unwrap(); (
                    let begin_node: u32, let end_node: u32,
                    let component_count: u32
            ));

            let mut components = Vec::<Component>::with_capacity(component_count as usize);

            for _ in 0 .. component_count {
                let_scan!(&lines.next().unwrap().unwrap(); (
                        let r#type: char, let value: f64
                ));

                components.push(match r#type {
                    'v' => Component::VoltageSource(value),
                    'i' => Component::CurrentSource(value),
                    'r' => Component::Resistor(value),

                    _ => panic!("invalid component type {}", r#type),
                });
            }

            wires.push(Wire {
                begin_node,
                end_node,
                components,
            });
        }

        Circuit {
            node_count,
            wires,
        }
    }
}
