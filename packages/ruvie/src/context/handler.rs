use std::marker::PhantomData;

pub struct Event<E, T> {
	pub event: E,
	pub(crate) _t: PhantomData<T>,
}
