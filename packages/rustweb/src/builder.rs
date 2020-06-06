use crate::{
	component::{Component, Lifecycle},
	context::Render,
	element::TypedElement,
	func::Func,
	props::{PropFor, Props},
	reference::{CompatibleReference, Reference},
	target::Target,
	Children, Element,
};

use std::{hash::Hash, marker::PhantomData, rc::Rc};

pub struct ElementBuilder<C, T>
where
	C: Component<T>,
	T: Target,
{
	component: PhantomData<C>,
	props: C::Props,
	children: Children<T>,
	reference: Option<Reference<T>>,
}

impl<C, T> ElementBuilder<C, T>
where
	C: Component<T>,
	T: Target,
{
	pub fn new(props: C::Props) -> Self {
		ElementBuilder {
			component: PhantomData,
			props,
			children: None.into(),
			reference: None,
		}
	}

	pub fn with_ref(mut self, refr: impl CompatibleReference<C, T>) -> Self {
		self.reference = Some(refr.to_bound_ref());
		self
	}

	pub fn prop<P: PropFor<C> + Hash, V: Into<P::Value>>(mut self, prop: P, value: V) -> Self
	where
		C: Component<T, Props = Rc<Props<C>>>,
	{
		let props = Rc::get_mut(&mut self.props).unwrap();
		props.value_for(prop, value.into());
		self
	}

	pub fn child<IL>(mut self, child: IL) -> Self
	where
		IL: Into<Element<T>>,
	{
		let element = child.into();
		match self.children.as_mut() {
			Some(children) => children.push(element),
			None => self.children = Children::from(element),
		}

		self
	}

	pub fn scope<FU, IC>(mut self, child: FU) -> Self
	where
		FU: Fn(&Render<T>) -> IC + 'static,
		IC: Into<Children<T>>,
	{
		let element = Func::build(move |ctx| child(ctx).into());
		match self.children.as_mut() {
			Some(children) => children.push(element),
			None => self.children = Children::from(element),
		}
		self
	}

	pub fn children(mut self, mut children: Children<T>) -> Self {
		if children.is_none() {
			return self;
		}

		match self.children.as_mut() {
			Some(current) => current.append(children.as_mut().unwrap()),
			None => self.children = children,
		}

		self
	}

	pub fn build(self) -> Element<T> {
		Element::from(TypedElement::<C, T> {
			component: PhantomData,
			children: self.children,
			reference: self.reference,
			props: self.props,
		})
	}
}

impl<C, T> From<ElementBuilder<C, T>> for Children<T>
where
	C: Component<T> + Lifecycle<T>,
	T: Target,
{
	fn from(builder: ElementBuilder<C, T>) -> Self {
		Children::from(builder.build())
	}
}
