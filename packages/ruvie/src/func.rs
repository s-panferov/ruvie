use std::{marker::PhantomData, sync::Arc};

use crate::children::Children;
use crate::{
	builder::{ElementBuilder, Factory},
	context::Render,
	Component, Scope,
};

pub trait FunctionExt: Fn(&Render) -> Children + Sized {
	fn default(self) -> ElementBuilder<Func<Self>> {
		ElementBuilder::new(
			Box::new(Func {
				func: Arc::new(self),
			}),
			Default::default(),
		)
	}
}

impl<F> FunctionExt for F where F: Fn(&Render) -> Children + Sized {}

pub struct Func<F>
where
	F: Fn(&Render) -> Children + 'static,
{
	func: Arc<F>,
}

impl<F> Func<F>
where
	F: Fn(&Render) -> Children + 'static,
{
	pub fn new(func: F) -> Self {
		Func {
			func: Arc::new(func),
		}
	}
}

impl<F> Factory<Self> for Func<F>
where
	F: Fn(&Render) -> Children + 'static,
{
	fn create(&self, _props: (), _scope: Scope<Self>) -> Self {
		Func {
			func: self.func.clone(),
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

/* WITH PROPS */

pub struct FuncWithPropsFactory<F, P>
where
	F: Fn(P, &Render) -> Children + 'static,
{
	func: Arc<F>,
	props: PhantomData<P>,
}

pub struct FuncWithProps<F, P>
where
	F: Fn(P, &Render) -> Children + 'static,
{
	func: Arc<F>,
	props: Option<P>,
}

impl<F, P> Factory<FuncWithProps<F, P>> for FuncWithPropsFactory<F, P>
where
	F: Fn(P, &Render) -> Children + 'static,
	P: Clone + 'static,
{
	fn create(&self, props: P, _scope: Scope<FuncWithProps<F, P>>) -> FuncWithProps<F, P> {
		FuncWithProps {
			func: self.func.clone(),
			props: Some(props),
		}
	}
}

pub trait FunctionWithPropsExt<P: Clone + 'static>: Fn(P, &Render) -> Children + Sized {
	fn with_props(self, props: P) -> ElementBuilder<FuncWithProps<Self, P>> {
		ElementBuilder::new(
			Box::new(FuncWithPropsFactory {
				func: Arc::new(self),
				props: PhantomData,
			}),
			props,
		)
	}

	fn default(self) -> ElementBuilder<FuncWithProps<Self, P>>
	where
		P: Default,
	{
		ElementBuilder::new(
			Box::new(FuncWithPropsFactory {
				func: Arc::new(self),
				props: PhantomData,
			}),
			Default::default(),
		)
	}
}

impl<F, P: Clone + 'static> FunctionWithPropsExt<P> for F where F: Fn(P, &Render) -> Children + Sized
{}

impl<F, P> Component for FuncWithProps<F, P>
where
	F: Fn(P, &Render) -> Children + 'static,
	P: Clone + 'static,
{
	type Props = P;

	fn name() -> &'static str {
		"Function"
	}

	fn render(&mut self, ctx: &Render) -> Children {
		(self.func)(self.props.take().unwrap(), ctx)
	}
}
