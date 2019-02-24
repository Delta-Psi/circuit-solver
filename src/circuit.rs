mod read;
mod solve;

/// Each implemented component of an electric circuit.
#[derive(Debug)]
pub enum Component {
    /// Creates a constant voltage difference.
    VoltageSource(f64),
    /// Creates a constant current.
    CurrentSource(f64),
    /// Drops the voltage proportionally to the current.
    Resistor(f64),
}

/// A connection between two nodes, consisting of
/// serially connected components.
#[derive(Debug)]
pub struct Wire {
    pub begin_node: u32,
    pub end_node: u32,

    pub components: Vec<Component>,
}

/// A collection of nodes connected by wires.
#[derive(Debug)]
pub struct Circuit {
    pub node_count: u32,
    pub wires: Vec<Wire>,
}
