use web_sys::Document;

use super::{event::BoxedWebHandler, fragment::FragmentBuilder, Web};

use crate::context::Mount;
use std::ops::{Deref, DerefMut};

pub struct WebContext {
	pub doc: Document,
	pub(crate) fragment: FragmentBuilder,
	pub(crate) handlers: Vec<Box<dyn BoxedWebHandler>>,
	pub(crate) mount: Mount<Web>,
}

impl Deref for WebContext {
	type Target = Mount<Web>;
	fn deref(&self) -> &Self::Target {
		&self.mount
	}
}

impl DerefMut for WebContext {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.mount
	}
}

impl WebContext {
	pub fn handler(&mut self, handler: Box<dyn BoxedWebHandler>) {
		self.handlers.push(handler);
	}
}
