use crate::{target::Target, view::ReactionCallback};

pub struct AfterRender<T>
where
	T: Target,
{
	pub(crate) reactions: Vec<ReactionCallback<T>>,
}

impl<T> AfterRender<T>
where
	T: Target,
{
	pub fn reaction(&mut self, handler: ReactionCallback<T>) {
		self.reactions.push(handler);
	}
}
