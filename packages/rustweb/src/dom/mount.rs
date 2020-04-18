use web_sys::Document;
use web_sys::Node;

pub struct HtmlMount {
    pub doc: Document,
    pub(crate) nodes: Vec<Node>,
}

impl HtmlMount {
    pub(crate) fn add_node(&mut self, node: &Node) {
        self.nodes.push(node.clone());
    }
}
