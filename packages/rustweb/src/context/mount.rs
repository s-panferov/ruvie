use crate::target::Target;
use crate::view::{ReactionCallback, View};
use crate::Children;

pub struct Mount<T: Target + ?Sized> {
	pub(crate) children: Vec<View<T>>,
	pub(crate) tree: Children<T>,
	pub(crate) reactions: Vec<ReactionCallback<T>>,
	pub(crate) view: View<T>,
}

impl<T> Mount<T>
where
	T: Target,
{
	pub(crate) fn add_child(&mut self, child: View<T>) {
		self.children.push(child);
	}

	pub fn reaction(&mut self, handler: ReactionCallback<T>) {
		self.reactions.push(handler);
	}
}
