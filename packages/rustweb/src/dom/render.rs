use std::sync::Arc;
use wasm_bindgen::JsValue;
use web_sys::{HtmlElement, Node};

use crate::component::Component;
use crate::component::UpdateContext;
use crate::{
    instance::{InstanceOptions, InstanceRef},
    layout::Layout,
    scheduler::Scheduler,
};

use super::Html;

pub fn node<P>(ctx: &UpdateContext<P, Html>) -> Option<Node> {
    ctx.instance
        .runtime
        .as_ref()
        .and_then(|r| r.nodes.get(0))
        .map(|el| el.clone())
}

pub fn render<C: Component<Target = Html>>(
    el: HtmlElement,
    layout: Layout<C>,
) -> Result<InstanceRef<Html>, JsValue> {
    console_error_panic_hook::set_once();

    let scheduler = Scheduler::new();
    let instance = InstanceRef::new(InstanceOptions {
        scheduler: scheduler.clone(),
        parent: None,
        layout: Arc::new(layout),
        level: 0,
    });

    let res = instance.perform_render()?;
    el.append_child(&res)?;

    Ok(instance)
}
