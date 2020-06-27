use crate::view::{ReactionCallback, View};
use crate::Children;

pub struct Mount {
	pub(crate) children: Vec<View>,
	pub(crate) tree: Children,
	pub(crate) reactions: Vec<ReactionCallback>,
	pub(crate) view: View,
}

impl Mount {
	pub(crate) fn add_child(&mut self, child: View) {
		self.children.push(child);
	}

	pub fn reaction(&mut self, handler: ReactionCallback) {
		self.reactions.push(handler);
	}
}
