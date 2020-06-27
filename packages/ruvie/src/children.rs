use std::ops::{Deref, DerefMut};

use crate::Element;

#[derive(Clone)]
pub struct Children {
	children: Option<Vec<Element>>,
}

impl Children {
	pub fn take(&mut self) -> Children {
		Children {
			children: self.children.take(),
		}
	}

	pub fn unwrap(self) -> Vec<Element> {
		self.children.unwrap()
	}
}

impl Deref for Children {
	type Target = Option<Vec<Element>>;
	fn deref(&self) -> &Self::Target {
		&self.children
	}
}

impl DerefMut for Children {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.children
	}
}

impl From<Option<Vec<Element>>> for Children {
	fn from(children: Option<Vec<Element>>) -> Self {
		Children { children }
	}
}

impl From<Vec<Element>> for Children {
	fn from(children: Vec<Element>) -> Self {
		Children {
			children: Some(children),
		}
	}
}

impl From<Element> for Children {
	fn from(children: Element) -> Self {
		Children {
			children: Some(vec![children]),
		}
	}
}
