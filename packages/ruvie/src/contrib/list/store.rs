use super::{IndexList, IndexListKey, ListProps};
use crate::target::Target;
use std::sync::Arc;

pub struct ListStore<K, V, T>
where
	K: IndexListKey,
	V: 'static,
	T: Target,
{
	pub props: ListProps<K, V, T>,
	pub prev: Arc<IndexList<K, V>>,
}

impl<K, V, T> ListStore<K, V, T>
where
	K: IndexListKey,
	T: Target,
{
	pub fn new(props: ListProps<K, V, T>) -> Self {
		Self {
			props,
			prev: Arc::new(IndexList::new()),
		}
	}
}
