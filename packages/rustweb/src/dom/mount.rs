use wasm_bindgen::{convert::FromWasmAbi, JsValue};
use web_sys::Document;
use web_sys::Node;

use super::event::{bind, BoxedHandler};
use crate::event::Event;

pub struct HtmlMount {
    pub doc: Document,
    pub(crate) nodes: Vec<Node>,
    pub(crate) events: Vec<Box<dyn BoxedHandler>>,
}

impl HtmlMount {
    pub(crate) fn add_node(&mut self, node: &Node) {
        self.nodes.push(node.clone());
    }

    pub fn bind_event<E: FromWasmAbi>(
        &mut self,
        value: Event<E>,
        node: Node,
        ev: &str,
    ) -> Result<(), JsValue>
    where
        E: 'static,
    {
        let handler = bind(value, node, ev)?;
        self.events.push(Box::new(handler));
        Ok(())
    }
}
