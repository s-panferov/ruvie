use super::{target::StaticElementState, StaticContext};
use crate::{context::Mount, error::RuvieError, view::View};
use html5ever::serialize;
use markup5ever_rcdom::{Node, NodeData, SerializableHandle};
use serialize::SerializeOpts;
use std::{
	cell::{Cell, RefCell},
	hash::Hash,
	rc::Rc,
};

pub fn mount_children(
	ctx: &mut Mount,
	target: &mut StaticContext,
	node: Option<&Node>,
) -> Result<(), RuvieError> {
	ctx.children::<StaticElementState, _, _>(|_ctx, state| match node {
		Some(node) => {
			let mut children = node.children.borrow_mut();
			children.append(&mut state.fragment.clone());
			Ok(())
		}
		None => {
			target.fragment.append(&mut state.fragment.clone());
			Ok(())
		}
	})
}

pub fn stringify(view: &View) -> String {
	let mut value = Vec::new();
	let document: SerializableHandle = view.with_state(|state| {
		let state = state
			.as_ref()
			.unwrap()
			.downcast_ref::<StaticElementState>()
			.unwrap();

		Rc::new(Node {
			parent: Cell::new(None),
			children: RefCell::new(state.fragment.clone()),
			data: NodeData::Document,
		})
		.into()
	});

	serialize(
		&mut value,
		&document,
		SerializeOpts {
			scripting_enabled: false,
			traversal_scope: serialize::TraversalScope::ChildrenOnly(None),
			create_missing_parent: false,
		},
	)
	.ok()
	.expect("serialization failed");

	String::from_utf8(value).unwrap()
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
