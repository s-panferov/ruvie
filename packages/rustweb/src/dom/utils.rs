use snowflake::ProcessUniqueId;
use wasm_bindgen::JsValue;
use web_sys::Node;

use super::{Html, HtmlMount};
use crate::{instance::Instance, render};

pub fn mount_children(ctx: &mut HtmlMount, into: &Node) -> Result<(), JsValue> {
    let children = ctx.tree.take();
    if children.is_some() {
        for layout in children.unwrap().into_iter() {
            let (child_el, instance) = render(layout, Some(ctx.instance.clone()))?;
            into.append_child(&child_el)?;
            ctx.add_child(instance);
        }
    }

    Ok(())
}

pub fn node(ctx: &Instance<Html>, r: &ProcessUniqueId) -> Option<Node> {
    ctx.get(r).and_then(|inst| {
        inst.state_mut()
            .platform
            .as_ref()
            .and_then(|r| r.nodes.get(0))
            .map(|el| el.clone())
    })
}
