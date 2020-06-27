use crate::{
	component::{Component, Constructor},
	context::Render,
	element::TypedElement,
	func::Func,
	props::{PropFor, Props},
	reference::{CompatibleReference, Reference},
	Children, Element,
};

use std::{hash::Hash, marker::PhantomData, rc::Rc};

pub struct ElementBuilder<C>
where
	C: Component,
{
	component: PhantomData<C>,
	props: C::Props,
	children: Children,
	reference: Option<Reference>,
}

impl<C> ElementBuilder<C>
where
	C: Component,
{
	pub fn new(props: C::Props) -> Self {
		ElementBuilder {
			component: PhantomData,
			props,
			children: None.into(),
			reference: None,
		}
	}

	pub fn with_ref(mut self, refr: impl CompatibleReference<C>) -> Self {
		self.reference = Some(refr.to_bound_ref());
		self
	}

	pub fn prop<P: PropFor<C> + Hash, V: Into<P::Value>>(mut self, prop: P, value: V) -> Self
	where
		C: Component<Props = Rc<Props<C>>>,
	{
		let props = Rc::get_mut(&mut self.props).unwrap();
		props.value_for(prop, value.into());
		self
	}

	pub fn child<IL>(mut self, child: IL) -> Self
	where
		IL: Into<Element>,
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
		FU: Fn(&Render) -> IC + 'static,
		IC: Into<Children>,
	{
		let element = Func::build(move |ctx| child(ctx).into());
		match self.children.as_mut() {
			Some(children) => children.push(element),
			None => self.children = Children::from(element),
		}
		self
	}

	pub fn children(mut self, mut children: Children) -> Self {
		if children.is_none() {
			return self;
		}

		match self.children.as_mut() {
			Some(current) => current.append(children.as_mut().unwrap()),
			None => self.children = children,
		}

		self
	}

	pub fn build(self) -> Element
	where
		C: Constructor,
	{
		Element::from(TypedElement::<C> {
			component: PhantomData,
			children: self.children,
			reference: self.reference,
			props: self.props,
		})
	}
}

impl<C> From<ElementBuilder<C>> for Children
where
	C: Component + Constructor,
{
	fn from(builder: ElementBuilder<C>) -> Self {
		Children::from(builder.build())
	}
}
