use web_sys::Document;
use web_sys::Node;

use super::{event::BoxedHandler, Html};

use crate::context::Mount;
use std::ops::{Deref, DerefMut};

pub struct HtmlMount {
    pub doc: Document,
    pub(crate) nodes: Vec<Node>,
    pub(crate) handlers: Vec<Box<dyn BoxedHandler>>,
    pub(crate) mount: Mount<Html>,
}

impl Deref for HtmlMount {
    type Target = Mount<Html>;
    fn deref(&self) -> &Self::Target {
        &self.mount
    }
}

impl DerefMut for HtmlMount {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mount
    }
}

impl HtmlMount {
    pub fn add_node(&mut self, node: &Node) {
        self.nodes.push(node.clone());
    }

    pub fn add_handler(&mut self, handler: Box<dyn BoxedHandler>) {
        self.handlers.push(handler);
    }
}
