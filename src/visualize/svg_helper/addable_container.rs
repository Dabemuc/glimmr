use svg::node::element::Group;
use svg::{Document, node::Node};
/// Trait that allows adding SVG nodes to any container (Document or Group)
///
pub trait AddableContainer {
    fn add_node(self, node: impl Node) -> Self;
}

impl AddableContainer for Document {
    fn add_node(mut self, node: impl Node) -> Self {
        self.append(node);
        self
    }
}

impl AddableContainer for Group {
    fn add_node(mut self, node: impl Node) -> Self {
        self.append(node);
        self
    }
}
