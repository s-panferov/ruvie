use crate::children::Children;
use crate::{
	builder::ElementBuilder,
	context::{AfterRender, Render},
	instance::Instance,
	props::PropFor,
	scope::Scope,
	target::Target,
	Props,
};

use std::{hash::Hash, rc::Rc};

/// Basic system trait all components should implement
pub trait Component<T: Target>: Sized + 'static {
	type Props: Clone;

	fn create(props: Self::Props, scope: Scope<Self, T>) -> Self;

	/// Defines component name. Useful for debugging.
	fn name() -> &'static str {
		return "Component";
	}

	/// You can prevent render on prop changes by implementing this
	fn should_render(&self) -> bool {
		true
	}

	/// Main function that defines component layout
	fn render(&mut self, ctx: &Render<T>) -> Children<T> {
		ctx.children.clone()
	}
}

pub trait Lifecycle<T>
where
	T: Target,
{
	fn after_render(&mut self, _ctx: &mut AfterRender<T>) {}

	fn before_unmount(&mut self) {}

	fn debug(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "View")
	}

	fn mount(&mut self, ctx: &mut T::Mount) -> Result<(), T::Error> {
		T::mount_component(ctx)
	}
}

impl<T, C> Lifecycle<T> for C
where
	C: Component<T>,
	T: Target,
{
	default fn after_render(&mut self, _ctx: &mut AfterRender<T>) {}
	default fn before_unmount(&mut self) {}
	default fn mount(&mut self, ctx: &mut T::Mount) -> Result<(), T::Error> {
		T::mount_component(ctx)
	}
}

pub trait ComponentExt<T>: Component<T>
where
	T: Target,
{
	fn with_props(props: Self::Props) -> ElementBuilder<Self, T> {
		ElementBuilder::new(props)
	}

	fn prop<P: PropFor<Self> + Hash, V: Into<P::Value>>(
		prop: P,
		value: V,
	) -> ElementBuilder<Self, T>
	where
		Self: Component<T, Props = Rc<Props<Self>>>,
	{
		let mut props = Props::new();
		props.value_for(prop, value.into());
		ElementBuilder::new(Rc::new(props))
	}

	fn default() -> ElementBuilder<Self, T>
	where
		Self::Props: Default,
	{
		ElementBuilder::new(Default::default())
	}
}

impl<C, T> ComponentExt<T> for C
where
	C: Component<T>,
	T: Target,
{
}

impl<C, T> Instance<T> for C
where
	C: Component<T> + Lifecycle<T>,
	T: Target,
{
	fn name(&self) -> &'static str {
		C::name()
	}

	fn should_render(&self) -> bool {
		C::should_render(self)
	}

	fn render(&mut self, render: &Render<T>) -> Children<T> {
		C::render(self, render)
	}

	fn debug(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		C::debug(self, f)
	}

	fn mount(&mut self, ctx: &mut T::Mount) -> Result<(), T::Error> {
		C::mount(self, ctx)
	}

	fn after_render(&mut self, ctx: &mut AfterRender<T>) {
		C::after_render(self, ctx);
	}

	fn before_unmount(&mut self) {
		C::before_unmount(self);
	}
}
