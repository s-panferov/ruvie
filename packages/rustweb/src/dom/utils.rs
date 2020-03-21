use crate::{instance::InstanceSpec, Children, InstanceRef};

use super::{html::MountContext, Html};

use wasm_bindgen::JsValue;
use web_sys::Node;

pub fn mount_children(
    ctx: &mut MountContext,
    children: Children<Html>,
    into: &Node,
) -> Result<(), JsValue> {
    if children.is_some() {
        for c in children.unwrap().into_iter() {
            let instance = InstanceRef::new(InstanceSpec {
                parent: ctx.parent.clone(),
                scheduler: ctx.scheduler.clone(),
                layout: c,
                level: ctx.parent.as_ref().map(|p| p.level).unwrap_or(0) + 1,
            });

            let child_el = instance.perform_render()?;

            into.append_child(&child_el)?;
            ctx.add_child(instance);
        }
    }

    Ok(())
}
