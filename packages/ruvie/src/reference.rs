use crate::{view::WeakView, Component};
use std::marker::PhantomData;

pub struct TypedReference<C>
where
	C: Component,
{
	pub id: u64,
	pub parent: WeakView,
	pub _c: PhantomData<C>,
}

impl<C> Clone for TypedReference<C>
where
	C: Component,
{
	fn clone(&self) -> Self {
		TypedReference {
			id: self.id.clone(),
			parent: self.parent.clone(),
			_c: PhantomData,
		}
	}
}

impl<C> From<TypedReference<C>> for Reference
where
	C: Component,
{
	fn from(v: TypedReference<C>) -> Self {
		Reference {
			parent: v.parent,
			id: v.id,
		}
	}
}

#[derive(Clone)]
pub struct Reference {
	pub parent: WeakView,
	pub id: u64,
}

impl Reference {
	pub fn apply(&self, inst: WeakView) {
		if let Some(parent) = self.parent.upgrade() {
			parent.register_reference(&self.id, inst)
		}
	}
}

pub trait CompatibleReference<C>
where
	C: Component,
{
	fn to_bound_ref(self) -> Reference;
}

impl<C> CompatibleReference<C> for Reference
where
	C: Component,
{
	fn to_bound_ref(self) -> Reference {
		self
	}
}

impl<C> CompatibleReference<C> for TypedReference<C>
where
	C: Component,
{
	fn to_bound_ref(self) -> Reference {
		self.into()
	}
}
