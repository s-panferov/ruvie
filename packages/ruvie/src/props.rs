use downcast_rs::{impl_downcast, Downcast};
use std::{any::TypeId, collections::HashMap, marker::PhantomData, ops::Deref};

pub trait Prop: 'static {}

pub trait PropFor<C>: Prop + Downcast {}

impl_downcast!(PropFor<C>);

pub struct Props<C> {
	_c: PhantomData<C>,
	props: HashMap<TypeId, Box<dyn PropFor<C>>>,
}

impl<C> Deref for Props<C> {
	type Target = HashMap<TypeId, Box<dyn PropFor<C>>>;
	fn deref(&self) -> &Self::Target {
		&self.props
	}
}

impl<C> Props<C> {
	pub fn new() -> Self {
		Props {
			_c: PhantomData,
			props: HashMap::new(),
		}
	}

	pub fn value_for<P: PropFor<C>>(&mut self, value: P) -> &mut Self {
		let _ = self.props.insert(TypeId::of::<P>(), Box::new(value));
		self
	}
}

impl<C> Default for Props<C> {
	fn default() -> Self {
		Props {
			_c: PhantomData,
			props: HashMap::new(),
		}
	}
}
