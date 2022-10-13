
pub enum Identifier {
    Numeric(u32),
    String(string),
}

pub struct NodeId {
    /// The index for a namespace
    pub namespace: u16,
    /// The identifier for the node in the address space
    pub identifier: Identifier,
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.namespace != 0 {
            write!(f, "ns={};{}", self.namespace, self.identifier)
        } else {
            write!(f, "{}", self.identifier)
        }
    }
}