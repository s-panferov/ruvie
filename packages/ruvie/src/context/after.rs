use crate::view::ReactionHandler;

pub struct AfterRender {
	pub(crate) reactions: Vec<ReactionHandler>,
}

impl AfterRender {
	pub fn reaction(&mut self, handler: ReactionHandler) {
		self.reactions.push(handler);
	}
}
