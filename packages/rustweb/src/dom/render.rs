use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::Node;

use snowflake::ProcessUniqueId;

use crate::{
    instance::{Instance, InstanceSpec},
    runtime::Runtime,
    Child,
};

use super::Html;

pub fn node(ctx: &dyn AsRef<Instance<Html>>, r: &dyn AsRef<ProcessUniqueId>) -> Option<Node> {
    ctx.as_ref().get(r).and_then(|inst| {
        inst.state()
            .runtime
            .as_ref()
            .and_then(|r| r.nodes.get(0))
            .map(|el| el.clone())
    })
}

pub fn render(
    layout: Child<Html>,
    parent: Option<Rc<Instance<Html>>>,
) -> Result<(Node, Rc<Instance<Html>>), JsValue> {
    let instance = Instance::new(InstanceSpec {
        runtime: parent
            .as_ref()
            .map(|parent| parent.spec.runtime.clone())
            .unwrap_or_else(|| Runtime::new()),
        level: parent.as_ref().map(|p| p.spec.level).unwrap_or(0) + 1,
        parent: parent.map(|p| Rc::downgrade(&p)),
        layout,
    });
    let res = instance.perform_render()?;
    Ok((res, instance))
}
