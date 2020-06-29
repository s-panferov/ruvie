use std::{marker::PhantomData, sync::Arc};

use crate::children::Children;
use crate::component::{Component, Constructor};
use crate::reference::Reference;
use crate::{builder::DynFactory, instance::Instance, scope::Scope, view::WeakView};

pub struct TypedElement<C>
where
	C: Component,
{
	pub(crate) factory: DynFactory<C>,
	pub(crate) props: C::Props,
	pub(crate) children: Children,
	pub(crate) reference: Option<Reference>,
}

impl<C> TypedElement<C>
where
	C: Constructor,
{
	pub fn new(props: C::Props, _children: Children) -> Self {
		Self {
			factory: Box::new(PhantomData),
			props: props,
			reference: None,
			children: None.into(),
		}
	}
}

impl<C> From<TypedElement<C>> for Children
where
	C: Component,
{
	fn from(typed: TypedElement<C>) -> Self {
		Children::from(Element::from(typed))
	}
}

pub struct Element {
	pub(crate) inner: Arc<dyn ElementFactory>,
}

impl Clone for Element {
	fn clone(&self) -> Self {
		Element {
			inner: self.inner.clone(),
		}
	}
}

impl Element {
	pub fn instance(&self, view: WeakView) -> Box<dyn Instance> {
		self.inner.instance(view)
	}

	pub fn children(&self) -> Children {
		self.inner.children()
	}

	pub fn reference(&self) -> Option<Reference> {
		self.inner.reference()
	}
}

impl<C> From<TypedElement<C>> for Element
where
	C: Component,
{
	fn from(el: TypedElement<C>) -> Self {
		Element {
			inner: Arc::new(el),
		}
	}
}

pub trait ElementFactory {
	fn instance(&self, view: WeakView) -> Box<dyn Instance>;
	fn children(&self) -> Children {
		Children::from(None)
	}

	fn reference(&self) -> Option<Reference> {
		None
	}
}

impl<C> ElementFactory for TypedElement<C>
where
	C: Component + 'static,
{
	fn instance(&self, view: WeakView) -> Box<dyn Instance> {
		Box::new(self.factory.create(self.props.clone(), Scope::new(view)))
	}

	fn children(&self) -> Children {
		self.children.clone()
	}

	fn reference(&self) -> Option<Reference> {
		self.reference.clone()
	}
}
