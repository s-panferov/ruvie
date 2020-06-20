use std::{marker::PhantomData, sync::Arc};

use crate::children::Children;
use crate::{
	context::Render, element::ElementFactory, instance::Instance, target::Target, view::WeakView,
	Component, Element, Scope,
};

pub struct Func<F, T>
where
	F: Fn(&Render<T>) -> Children<T> + 'static,
	T: Target,
{
	func: Arc<F>,
	t: PhantomData<T>,
}

impl<F, T> Clone for Func<F, T>
where
	F: Fn(&Render<T>) -> Children<T> + 'static,
	T: Target,
{
	fn clone(&self) -> Self {
		Func {
			func: self.func.clone(),
			t: PhantomData,
		}
	}
}

impl<F, T> ElementFactory<T> for Func<F, T>
where
	F: Fn(&Render<T>) -> Children<T> + 'static,
	T: Target,
{
	fn instance(&self, _view: WeakView<T>) -> Box<dyn Instance<T>> {
		Box::new(self.clone())
	}
}

impl<F, T> Func<F, T>
where
	F: Fn(&Render<T>) -> Children<T> + 'static,
	T: Target,
{
	pub fn build(func: F) -> Element<T> {
		Element {
			inner: Arc::new(Func {
				func: Arc::new(func),
				t: PhantomData,
			}),
		}
	}
}

impl<F, T> Component<T> for Func<F, T>
where
	F: Fn(&Render<T>) -> Children<T> + 'static,
	T: Target,
{
	type Props = ();

	fn create(_props: Self::Props, _scope: Scope<Self, T>) -> Self {
		unreachable!()
	}

	fn name() -> &'static str {
		"Function"
	}

	fn render(&mut self, ctx: &Render<T>) -> Children<T> {
		(self.func)(ctx)
	}
}
