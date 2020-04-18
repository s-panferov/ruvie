use wasm_bindgen::JsValue;
use web_sys::Node;

use super::{render, Html};
use crate::mount::Mount;

pub fn mount_children(ctx: &mut Mount<Html>, into: &Node) -> Result<(), JsValue> {
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
