use crate::{
	context::{Mount, Render, Update},
	Children, Component, Element,
};

use crate::{component::Constructor, error::RuvieError, scope::Scope, View};

use std::{
	collections::HashMap,
	fmt::Debug,
	hash::{Hash, Hasher},
	ops::{Deref, DerefMut},
	sync::Arc,
};

use std::any::Any;

use observe::{Observable, Value};

#[cfg(feature = "web")]
mod diff;
#[cfg(feature = "web")]
mod iter;

#[cfg(feature = "web")]
use crate::{web::WebContext, web::WebElementState};

mod store;

pub use store::*;

use indexmap::IndexMap;

#[cfg(feature = "web")]
use diff::{diff_with_hint, DiffApply};

pub trait IndexListKey: Hash + Eq + Ord + Clone + Debug + 'static {}
impl<T> IndexListKey for T where T: Hash + Eq + Ord + Clone + Debug + 'static {}

#[derive(Clone)]
pub struct IndexList<K, V> {
	map: IndexMap<K, V>,
}

impl<K, V> IndexList<K, V> {
	fn new() -> Self {
		IndexList {
			map: IndexMap::new(),
		}
	}
}

impl<K, V> From<IndexMap<K, V>> for IndexList<K, V> {
	fn from(map: IndexMap<K, V>) -> Self {
		Self { map }
	}
}

impl<K, V> Hash for IndexList<K, V>
where
	K: Hash + Eq,
{
	fn hash<H: Hasher>(&self, state: &mut H) {
		for key in self.map.keys() {
			key.hash(state)
		}
	}
}

impl<K, V> Deref for IndexList<K, V> {
	type Target = IndexMap<K, V>;
	fn deref(&self) -> &Self::Target {
		&self.map
	}
}

impl<K, V> DerefMut for IndexList<K, V> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.map
	}
}

pub struct ListProps<K, V>
where
	K: IndexListKey,
	V: 'static,
{
	pub hint: ListHint,
	pub list: Value<Arc<IndexList<K, V>>>,
	pub item: Arc<dyn Fn(&K, &V) -> Element>,
}

impl<K, V> Clone for ListProps<K, V>
where
	K: IndexListKey,
	V: 'static,
{
	fn clone(&self) -> Self {
		ListProps {
			hint: self.hint.clone(),
			list: self.list.clone(),
			item: self.item.clone(),
		}
	}
}

pub struct List<K, V>
where
	K: IndexListKey + 'static,
	V: 'static,
{
	store: ListStore<K, V>,
	children: HashMap<K, View>,
	scope: Scope<Self>,
}

#[derive(PartialEq, Clone, Eq)]
pub enum ListHint {
	Append,
	Prepend,
	Pop,
	Shift,
	Random,
}

impl Default for ListHint {
	fn default() -> Self {
		ListHint::Random
	}
}

#[cfg(feature = "web")]
impl<K, V> Constructor for List<K, V>
where
	K: IndexListKey + 'static,
	V: 'static,
{
	fn create(props: Self::Props, scope: Scope<Self>) -> Self {
		let store = ListStore::new(props);
		Self {
			store,
			scope,
			children: HashMap::new(),
		}
	}
}

impl<K, V> Component for List<K, V>
where
	K: IndexListKey + 'static,
	V: 'static,
{
	type Props = ListProps<K, V>;

	fn name() -> &'static str {
		"List"
	}

	fn should_render(&self) -> bool {
		false
	}

	fn render(&mut self, _ctx: &Render) -> Children {
		let store = &mut self.store;

		let mut children = vec![];
		let list = store.props.list.once().clone();
		for (k, v) in list.iter() {
			let item = (store.props.item)(k, v);
			children.push(item)
		}

		store.prev = list;
		children.into()
	}

	fn mount(&mut self, ctx: &mut Mount, target: &mut dyn Any) -> Result<(), RuvieError> {
		if ctx.tree.is_none() {
			return Ok(());
		}

		let list = self.store.props.list.once();
		let children = ctx.tree.take();
		for ((k, _v), elem) in list.iter().zip(children.unwrap().into_iter()) {
			let child = ctx.view.render_child(elem, None)?;

			#[cfg(feature = "web")]
			if target.is::<WebContext>() {
				let target = target.downcast_mut::<WebContext>().unwrap();
				child.with_state(|state| {
					let state = state
						.as_mut()
						.unwrap()
						.downcast_ref::<WebElementState>()
						.unwrap();
					let child_fragment = &state.fragment;
					target.fragment.child(child_fragment.clone());
					Ok::<(), wasm_bindgen::JsValue>(())
				})?;
			}

			self.children.insert(k.clone(), child);
		}

		Ok(())
	}

	#[cfg(feature = "web")]
	fn after_render(&mut self, ctx: &mut crate::context::AfterRender) {
		let reaction = self
			.scope
			.reaction(move |this: &mut Self, ctx: &mut Update| {
				let next = this.store.props.list.get(ctx.eval).clone();
				let hint = this.store.props.hint.clone();
				let prev = this.store.prev.clone();

				let mut diff_cb = DiffApply {
					view: ctx.view,
					list: this,
					next: &next,
				};

				diff_with_hint(&prev, &next, hint, &mut diff_cb);
				this.store.prev = next;

				Ok(())
			});

		ctx.reaction(reaction);
	}

	fn before_unmount(&mut self) {}
	fn debug(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "View")
	}
}
