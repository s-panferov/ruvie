use std::ops::{Deref, DerefMut};

use crate::{target::Target, Element};

#[derive(Clone)]
pub struct Children<T>
where
	T: Target + ?Sized,
{
	children: Option<Vec<Element<T>>>,
}

impl<T> Children<T>
where
	T: Target,
{
	pub fn take(&mut self) -> Children<T> {
		Children {
			children: self.children.take(),
		}
	}

	pub fn unwrap(self) -> Vec<Element<T>> {
		self.children.unwrap()
	}
}

impl<T> Deref for Children<T>
where
	T: Target,
{
	type Target = Option<Vec<Element<T>>>;
	fn deref(&self) -> &Self::Target {
		&self.children
	}
}

impl<T> DerefMut for Children<T>
where
	T: Target,
{
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.children
	}
}

impl<T> From<Option<Vec<Element<T>>>> for Children<T>
where
	T: Target,
{
	fn from(children: Option<Vec<Element<T>>>) -> Self {
		Children { children }
	}
}

impl<T> From<Vec<Element<T>>> for Children<T>
where
	T: Target,
{
	fn from(children: Vec<Element<T>>) -> Self {
		Children {
			children: Some(children),
		}
	}
}

impl<T> From<Element<T>> for Children<T>
where
	T: Target,
{
	fn from(children: Element<T>) -> Self {
		Children {
			children: Some(vec![children]),
		}
	}
}
