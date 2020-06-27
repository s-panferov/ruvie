use crate::view::ReactionCallback;

pub struct AfterRender {
	pub(crate) reactions: Vec<ReactionCallback>,
}

impl AfterRender {
	pub fn reaction(&mut self, handler: ReactionCallback) {
		self.reactions.push(handler);
	}
}
