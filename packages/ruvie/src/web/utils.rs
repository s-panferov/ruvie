use wasm_bindgen::JsValue;
use web_sys::Node;

use super::{fragment::ChildPosition, target::WebElementState, WebContext};
use crate::{context::Mount, error::RuvieError, view::View};
use std::hash::Hash;

pub fn mount_children(
	ctx: &mut Mount,
	target: &mut WebContext,
	node: Option<&Node>,
) -> Result<(), RuvieError> {
	if ctx.tree.is_none() {
		return Ok(());
	}

	let children = ctx.tree.take();
	for element in children.unwrap().into_iter() {
		let child = ctx.view.render_child(element, None)?;
		child.with_state(|state| {
			let state = state
				.as_mut()
				.unwrap()
				.downcast_mut::<WebElementState>()
				.unwrap();
			let child_fragment = &state.fragment;
			if let Some(node) = node {
				child_fragment
					.borrow_mut()
					.insert_self(node, ChildPosition::Append)?
			} else {
				target.fragment.child(child_fragment.clone())
			}
			Ok::<(), JsValue>(())
		})?;

		ctx.add_child(child);
	}

	Ok(())
}

pub fn node<K: Hash>(_ctx: &View, _r: &K) -> Option<Node> {
	todo!()
	// ctx.get(r).and_then(|inst| {
	// 	inst.state_mut()
	// 		.target
	// 		.as_ref()
	// 		.and_then(|r| r.fragment.borrow().left())
	// 		.map(|el| el.clone())
	// })
}
