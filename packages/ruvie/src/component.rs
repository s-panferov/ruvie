use std::{any::Any, hash::Hash, marker::PhantomData, rc::Rc};

use crate::children::Children;
use crate::{
	builder::ElementBuilder,
	context::{AfterRender, Mount, Render},
	error::RuvieError,
	instance::Instance,
	props::PropFor,
	scope::Scope,
	Props,
};

pub trait Constructor: Component {
	fn create(props: Self::Props, scope: Scope<Self>) -> Self;
}

/// Basic system trait all components should implement
pub trait Component: Sized + 'static {
	type Props: Clone;

	/// Defines component name. Useful for debugging.
	fn name() -> &'static str {
		return "Component";
	}

	/// You can prevent render on prop changes by implementing this
	fn should_render(&self) -> bool {
		true
	}

	/// Main function that defines component layout
	fn render(&mut self, ctx: &Render) -> Children {
		ctx.children.clone()
	}

	fn after_render(&mut self, _ctx: &mut AfterRender) {}

	fn before_unmount(&mut self) {}

	fn debug(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "View")
	}

	fn mount(&mut self, ctx: &mut Mount, target: &mut dyn Any) -> Result<(), RuvieError> {
		// FIXME replace .clone() with a Platform::mount_component
		ctx.view
			.def()
			.runtime
			.platform
			.clone()
			.mount_component(ctx, target)
	}
}

pub trait ComponentExt: Constructor {
	fn with_props(props: Self::Props) -> ElementBuilder<Self> {
		ElementBuilder::new(Box::new(PhantomData), props)
	}

	fn prop<P: PropFor<T> + Hash, V: Into<P::Value>, T>(prop: P, value: V) -> ElementBuilder<Self>
	where
		Self: Component<Props = Rc<Props<T>>>,
	{
		let mut props = Props::new();
		props.value_for(prop, value.into());
		ElementBuilder::new(Box::new(PhantomData), Rc::new(props))
	}

	fn dynamic<T>() -> ElementBuilder<Self>
	where
		Self: Component<Props = Rc<Props<T>>>,
	{
		let props = Props::new();
		ElementBuilder::new(Box::new(PhantomData), Rc::new(props))
	}

	fn default() -> ElementBuilder<Self>
	where
		Self::Props: Default,
	{
		ElementBuilder::new(Box::new(PhantomData), Default::default())
	}
}

impl<C> ComponentExt for C where C: Constructor {}

impl<C> Instance for C
where
	C: Component,
{
	fn name(&self) -> &'static str {
		C::name()
	}

	fn should_render(&self) -> bool {
		C::should_render(self)
	}

	fn render(&mut self, render: &Render) -> Children {
		C::render(self, render)
	}

	fn debug(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		C::debug(self, f)
	}

	fn mount(&mut self, ctx: &mut Mount, target: &mut dyn Any) -> Result<(), RuvieError> {
		C::mount(self, ctx, target)
	}

	fn after_render(&mut self, ctx: &mut AfterRender) {
		C::after_render(self, ctx);
	}

	fn before_unmount(&mut self) {
		C::before_unmount(self);
	}
}
