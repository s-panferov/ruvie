use wasm_bindgen::JsValue;
use web_sys::Node;

use super::{Html, HtmlMount};
use crate::instance::Instance;
use std::hash::Hash;

pub fn mount_children(ctx: &mut HtmlMount, into: &Node) -> Result<(), JsValue> {
	if ctx.tree.is_none() {
		return Ok(());
	}

	let children = ctx.tree.take();
	for layout in children.unwrap().into_iter() {
		let (child_el, instance) = ctx.render_child(layout)?;
		into.append_child(&child_el)?;
		ctx.add_child(instance);
	}

	Ok(())
}

pub fn node<K: Hash>(ctx: &Instance<Html>, r: &K) -> Option<Node> {
	ctx.get(r).and_then(|inst| {
		inst.state_mut()
			.target
			.as_ref()
			.and_then(|r| r.fragment.first())
			.map(|el| el.clone())
	})
}
