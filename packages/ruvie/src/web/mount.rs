use web_sys::Document;

use super::{event::BoxedWebHandler, fragment::FragmentBuilder};

pub struct WebContext {
	pub doc: Document,
	pub(crate) fragment: FragmentBuilder,
	pub(crate) handlers: Vec<Box<dyn BoxedWebHandler>>,
}

impl WebContext {
	pub fn handler(&mut self, handler: Box<dyn BoxedWebHandler>) {
		self.handlers.push(handler);
	}
}
