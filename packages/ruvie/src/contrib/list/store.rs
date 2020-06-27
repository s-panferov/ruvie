use super::{IndexList, IndexListKey, ListProps};
use std::sync::Arc;

pub struct ListStore<K, V>
where
	K: IndexListKey,
	V: 'static,
{
	pub props: ListProps<K, V>,
	pub prev: Arc<IndexList<K, V>>,
}

impl<K, V> ListStore<K, V>
where
	K: IndexListKey,
{
	pub fn new(props: ListProps<K, V>) -> Self {
		Self {
			props,
			prev: Arc::new(IndexList::new()),
		}
	}
}
