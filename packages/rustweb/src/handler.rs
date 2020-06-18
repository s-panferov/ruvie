use std::{fmt::Debug, rc::Rc};

pub struct Handler<T> {
	handler: Rc<dyn Fn(T)>,
}

impl<T> Clone for Handler<T> {
	fn clone(&self) -> Self {
		Handler {
			handler: self.handler.clone(),
		}
	}
}

impl<T> Debug for Handler<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Event")
	}
}

impl<T> Handler<T> {
	pub fn new<F>(handler: F) -> Handler<T>
	where
		F: Fn(T) + 'static,
	{
		Handler {
			handler: Rc::new(handler),
		}
	}

	pub fn trigger(&self, ev: T) {
		(self.handler)(ev)
	}
}
