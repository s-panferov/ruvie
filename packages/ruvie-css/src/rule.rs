use downcast_rs::{impl_downcast, Downcast};
use std::fmt::Display;

pub trait Attribute {
	const NAME: &'static str;
}

pub trait ValueFor<A: Attribute>: Downcast + Display {
	fn value(&self) -> String {
		self.to_string()
	}
}

impl_downcast!(ValueFor<A> where A: Attribute);
