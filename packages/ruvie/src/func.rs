use std::sync::Arc;

use crate::children::Children;
use crate::{
	context::Render, element::ElementFactory, instance::Instance, view::WeakView, Component,
	Element, Scope,
};

pub struct Func<F>
where
	F: Fn(&Render) -> Children + 'static,
{
	func: Arc<F>,
}

pub struct FuncP<F, P>
where
	F: Fn(&P, &Render) -> Children + 'static,
{
	func: Arc<F>,
	props: P,
}

impl<F> Clone for Func<F>
where
	F: Fn(&Render) -> Children + 'static,
{
	fn clone(&self) -> Self {
		Func {
			func: self.func.clone(),
		}
	}
}

impl<F, P> Clone for FuncP<F, P>
where
	F: Fn(&P, &Render) -> Children + 'static,
	P: Clone,
{
	fn clone(&self) -> Self {
		FuncP {
			func: self.func.clone(),
			props: self.props.clone(),
		}
	}
}

impl<F> ElementFactory for Func<F>
where
	F: Fn(&Render) -> Children + 'static,
{
	fn instance(&self, _view: WeakView) -> Box<dyn Instance> {
		Box::new(self.clone())
	}
}

impl<F> Func<F>
where
	F: Fn(&Render) -> Children + 'static,
{
	pub fn build(func: F) -> Element {
		Element {
			inner: Arc::new(Func {
				func: Arc::new(func),
			}),
		}
	}
}

impl<F> Component for Func<F>
where
	F: Fn(&Render) -> Children + 'static,
{
	type Props = ();

	fn name() -> &'static str {
		"Function"
	}

	fn render(&mut self, ctx: &Render) -> Children {
		(self.func)(ctx)
	}
}

impl<F, P> Component for FuncP<F, P>
where
	F: Fn(&P, &Render) -> Children + 'static,
	P: Clone + 'static,
{
	type Props = P;

	fn name() -> &'static str {
		"Function"
	}

	fn render(&mut self, ctx: &Render) -> Children {
		(self.func)(&self.props, ctx)
	}
}
