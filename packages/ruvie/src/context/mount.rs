use crate::view::{ReactionHandler, View};
use crate::{error::RuvieError, Children};

pub struct Mount {
	pub(crate) children: Vec<View>,
	pub(crate) tree: Children,
	pub(crate) reactions: Vec<ReactionHandler>,
	pub(crate) view: View,
}

impl Mount {
	pub(crate) fn add_child(&mut self, child: View) {
		self.children.push(child);
	}

	pub fn reaction(&mut self, handler: ReactionHandler) {
		self.reactions.push(handler);
	}

	pub fn children<S, R, F>(&mut self, mut func: F) -> Result<(), RuvieError>
	where
		F: FnMut(&mut Mount, &mut S) -> Result<R, RuvieError>,
		S: 'static,
	{
		if self.tree.is_none() {
			return Ok(());
		}

		let children = self.tree.take();
		for element in children.unwrap().into_iter() {
			let child = self.view.render_child(element, None)?;
			child.with_state(|state| {
				let state = state.as_mut().unwrap().downcast_mut::<S>().unwrap();
				func(self, state)
			})?;
			self.add_child(child);
		}

		Ok(())
	}
}
