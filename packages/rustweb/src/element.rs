use std::{marker::PhantomData, sync::Arc};

use crate::children::Children;
use crate::component::{Component, Lifecycle};
use crate::reference::Reference;
use crate::{instance::Instance, scope::Scope, target::Target, view::WeakView};

pub struct TypedElement<C, T>
where
	C: Component<T>,
	T: Target,
{
	pub(crate) component: PhantomData<C>,
	pub(crate) props: C::Props,
	pub(crate) children: Children<T>,
	pub(crate) reference: Option<Reference<T>>,
}

impl<C, T> TypedElement<C, T>
where
	C: Component<T>,
	T: Target,
{
	pub fn new(props: C::Props, _children: Children<T>) -> Self {
		Self {
			component: PhantomData,
			props: props,
			reference: None,
			children: None.into(),
		}
	}
}

impl<C, T> From<TypedElement<C, T>> for Children<T>
where
	C: Component<T> + Lifecycle<T>,
	T: Target,
{
	fn from(typed: TypedElement<C, T>) -> Self {
		Children::from(Element::from(typed))
	}
}

pub struct Element<T>
where
	T: Target,
{
	pub(crate) inner: Arc<dyn ElementFactory<T>>,
}

impl<T> Clone for Element<T>
where
	T: Target,
{
	fn clone(&self) -> Self {
		Element {
			inner: self.inner.clone(),
		}
	}
}

impl<T> Element<T>
where
	T: Target,
{
	pub fn instance(&self, view: WeakView<T>) -> Box<dyn Instance<T>> {
		self.inner.instance(view)
	}

	pub fn children(&self) -> Children<T> {
		self.inner.children()
	}

	pub fn reference(&self) -> Option<Reference<T>> {
		self.inner.reference()
	}
}

impl<C, T> From<TypedElement<C, T>> for Element<T>
where
	C: Component<T> + Lifecycle<T>,
	T: Target,
{
	fn from(el: TypedElement<C, T>) -> Self {
		Element {
			inner: Arc::new(el),
		}
	}
}

pub trait ElementFactory<T: Target> {
	fn instance(&self, view: WeakView<T>) -> Box<dyn Instance<T>>;
	fn children(&self) -> Children<T> {
		Children::from(None)
	}

	fn reference(&self) -> Option<Reference<T>> {
		None
	}
}

impl<C, T> ElementFactory<T> for TypedElement<C, T>
where
	C: Component<T> + Lifecycle<T> + 'static,
	T: Target,
{
	fn instance(&self, view: WeakView<T>) -> Box<dyn Instance<T>> {
		Box::new(C::create(self.props.clone(), Scope::new(view)))
	}

	fn children(&self) -> Children<T> {
		self.children.clone()
	}

	fn reference(&self) -> Option<Reference<T>> {
		self.reference.clone()
	}
}
