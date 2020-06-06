use crate::{
	context::{Handler, Update},
	reference::{Reference, TypedReference},
	target::Target,
	view::{ReactionCallback, WeakView},
	Component, Element, Event, View,
};

use std::{hash::Hash, marker::PhantomData, sync::Arc};

pub struct Scope<C, T>
where
	C: Component<T>,
	T: Target,
{
	_c: PhantomData<C>,
	view: WeakView<T>,
}

impl<C, T> Scope<C, T>
where
	C: Component<T>,
	T: Target,
{
	pub(crate) fn new(view: WeakView<T>) -> Self {
		Self {
			_c: PhantomData,
			view,
		}
	}

	pub fn child(&self, element: Element<T>, arg: Option<T::Arg>) -> Result<View<T>, T::Error> {
		match self.view.upgrade() {
			Some(view) => view.render_child(element, arg),
			None => panic!("View was dropped, cannot create a child"),
		}
	}

	/// Wrap a reaction callback to be run in the context of the component
	pub fn reaction<F>(&self, handler: F) -> ReactionCallback<T>
	where
		C: Component<T>,
		F: Fn(&mut C, &mut Update<T>) -> Result<(), T::Error> + 'static,
	{
		Box::new(move |view: &View<T>, ctx: &mut _| {
			view.with_instance(|component| {
				(handler)(component.downcast_mut::<C>().expect("Type"), ctx)
			})
		})
	}

	/// Wrap a reference
	fn reference_for<CH: Component<T>>(&self) -> TypedReference<CH, T> {
		TypedReference {
			id: fxhash::hash64(&snowflake::ProcessUniqueId::new()),
			parent: self.view.clone(),
			_c: PhantomData,
		}
	}

	pub fn reference<K: Hash>(&self, reference: &K) -> Reference<T> {
		Reference {
			parent: self.view.clone(),
			id: fxhash::hash64(&reference),
		}
	}

	/// Wrap a handler function to create an Event object that can
	/// sent to another component as an event handler
	pub fn handler<F, E>(&self, handler: F) -> Event<E>
	where
		F: Fn(&mut C, Handler<E, T>) + 'static,
		E: 'static,
	{
		let view = self.view.clone();
		let handler = Arc::new(handler);
		Event::new(move |event| {
			if let Some(view) = view.upgrade() {
				let handler = handler.clone();
				view.with_instance(move |component| {
					handler(
						component.downcast_mut::<C>().unwrap(),
						Handler {
							event,
							_t: PhantomData,
						},
					);
				})
			}
		})
	}
}
