use crate::{target::Target, view::WeakView, Component};
use std::marker::PhantomData;

pub struct TypedReference<C, T>
where
	C: Component<T>,
	T: Target,
{
	pub id: u64,
	pub parent: WeakView<T>,
	pub _c: PhantomData<C>,
}

impl<C, T> Clone for TypedReference<C, T>
where
	C: Component<T>,
	T: Target,
{
	fn clone(&self) -> Self {
		TypedReference {
			id: self.id.clone(),
			parent: self.parent.clone(),
			_c: PhantomData,
		}
	}
}

impl<C, T> From<TypedReference<C, T>> for Reference<T>
where
	C: Component<T>,
	T: Target,
{
	fn from(v: TypedReference<C, T>) -> Self {
		Reference {
			parent: v.parent,
			id: v.id,
		}
	}
}

#[derive(Clone)]
pub struct Reference<T: Target> {
	pub parent: WeakView<T>,
	pub id: u64,
}

impl<T> Reference<T>
where
	T: Target,
{
	pub fn apply(&self, inst: WeakView<T>) {
		if let Some(parent) = self.parent.upgrade() {
			parent.register_reference(&self.id, inst)
		}
	}
}

pub trait CompatibleReference<C, T>
where
	C: Component<T>,
	T: Target,
{
	fn to_bound_ref(self) -> Reference<T>;
}

impl<C, T> CompatibleReference<C, T> for Reference<T>
where
	C: Component<T>,
	T: Target,
{
	fn to_bound_ref(self) -> Reference<T> {
		self
	}
}

impl<C, T> CompatibleReference<C, T> for TypedReference<C, T>
where
	C: Component<T>,
	T: Target,
{
	fn to_bound_ref(self) -> Reference<T> {
		self.into()
	}
}
