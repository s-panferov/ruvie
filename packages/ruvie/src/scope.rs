use crate::{
	context::{Event, Update},
	error::RuvieError,
	reference::{Reference, TypedReference},
	view::{ReactionCallback, WeakView},
	Component, Element, Handler, View,
};

use std::{any::Any, hash::Hash, marker::PhantomData, sync::Arc};

pub struct Scope<C>
where
	C: Component,
{
	_c: PhantomData<C>,
	view: WeakView,
}

impl<C> Scope<C>
where
	C: Component,
{
	pub(crate) fn new(view: WeakView) -> Self {
		Self {
			_c: PhantomData,
			view,
		}
	}

	pub fn child(&self, element: Element, arg: Option<Box<dyn Any>>) -> Result<View, RuvieError> {
		match self.view.upgrade() {
			Some(view) => view.render_child(element, arg),
			None => panic!("View was dropped, cannot create a child"),
		}
	}

	/// Wrap a reaction callback to be run in the context of the component
	pub fn reaction<F>(&self, handler: F) -> ReactionCallback
	where
		C: Component,
		F: Fn(&mut C, &mut Update) -> Result<(), RuvieError> + 'static,
	{
		Box::new(move |view: &View, ctx: &mut _| {
			view.with_instance(|component| {
				(handler)(component.downcast_mut::<C>().expect("Type"), ctx)
			})
		})
	}

	/// Wrap a reference
	#[allow(unused)]
	fn reference_for<CH: Component>(&self) -> TypedReference<CH> {
		TypedReference {
			id: fxhash::hash64(&snowflake::ProcessUniqueId::new()),
			parent: self.view.clone(),
			_c: PhantomData,
		}
	}

	pub fn reference<K: Hash>(&self, reference: &K) -> Reference {
		Reference {
			parent: self.view.clone(),
			id: fxhash::hash64(&reference),
		}
	}

	/// Wrap a handler function to create an Event object that can
	/// sent to another component as an event handler
	pub fn handler<F, E>(&self, handler: F) -> Handler<E>
	where
		F: Fn(&mut C, Event<E>) + 'static,
		E: 'static,
	{
		let view = self.view.clone();
		let handler = Arc::new(handler);
		Handler::new(move |event| {
			if let Some(view) = view.upgrade() {
				let handler = handler.clone();
				view.with_instance(move |component| {
					handler(component.downcast_mut::<C>().unwrap(), Event { event });
				})
			}
		})
	}
}
