use crate::{
	component::{Component, Constructor},
	context::Render,
	element::TypedElement,
	func::Func,
	props::{PropFor, Props},
	reference::{CompatibleReference, Reference},
	Children, Element, Scope,
};

use std::{hash::Hash, marker::PhantomData, rc::Rc};

pub trait Factory<C: Component> {
	fn create(&self, props: C::Props, scope: Scope<C>) -> C;
}

impl<C: Constructor> Factory<C> for PhantomData<C> {
	fn create(&self, props: C::Props, scope: Scope<C>) -> C {
		C::create(props, scope)
	}
}

pub type DynFactory<C> = Box<dyn Factory<C>>;

pub struct ElementBuilder<C>
where
	C: Component,
{
	factory: DynFactory<C>,
	props: C::Props,
	children: Children,
	reference: Option<Reference>,
}

impl<C> ElementBuilder<C>
where
	C: Component,
{
	pub fn new(factory: DynFactory<C>, props: C::Props) -> Self {
		ElementBuilder {
			factory,
			props,
			children: None.into(),
			reference: None,
		}
	}

	pub fn with_ref(mut self, refr: impl CompatibleReference<C>) -> Self {
		self.reference = Some(refr.to_bound_ref());
		self
	}

	pub fn prop<P: PropFor<T> + Hash, V: Into<P::Value>, T>(mut self, prop: P, value: V) -> Self
	where
		C: Component<Props = Rc<Props<T>>>,
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
		let func = Func::new(move |ctx| child(ctx).into());
		let element = Element::from(TypedElement {
			factory: Box::new(func),
			children: None.into(),
			reference: None,
			props: (),
		});

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

	pub fn build(self) -> Element {
		Element::from(TypedElement::<C> {
			factory: self.factory,
			children: self.children,
			reference: self.reference,
			props: self.props,
		})
	}
}

impl<C: Component> From<ElementBuilder<C>> for Element {
	fn from(el: ElementBuilder<C>) -> Self {
		el.build()
	}
}

impl<C> From<ElementBuilder<C>> for Children
where
	C: Component,
{
	fn from(builder: ElementBuilder<C>) -> Self {
		Children::from(builder.build())
	}
}
