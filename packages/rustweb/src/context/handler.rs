use std::marker::PhantomData;

pub struct Handler<E, T> {
	pub event: E,
	pub(crate) _t: PhantomData<T>,
}
