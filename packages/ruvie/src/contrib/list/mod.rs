use crate::{
	context::{Render, Update},
	target::Target,
	Children, Component, Element,
};

#[cfg(feature = "web")]
use crate::{
	component::Lifecycle,
	scope::Scope,
	web::{
		fragment::{ChildPosition, FragmentChild},
		Web, WebContext,
	},
	View,
};

use std::{
	collections::HashMap,
	fmt::Debug,
	hash::{Hash, Hasher},
	ops::{Deref, DerefMut},
	sync::Arc,
};

use wasm_bindgen::JsValue;

use std::{
	borrow::Cow,
	cmp::Ordering::{self},
};

use observe::{Observable, Value};

mod iter;
mod store;

pub use store::*;

use indexmap::IndexMap;
use indexmap::IndexSet;

use iter::IteratorExt;

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

pub struct ListProps<K, V, T>
where
	T: Target,
	K: IndexListKey,
	V: 'static,
{
	pub hint: ListHint,
	pub list: Value<Arc<IndexList<K, V>>>,
	pub item: Arc<dyn Fn(&K, &V) -> Element<T>>,
}

impl<K, V, T> Clone for ListProps<K, V, T>
where
	T: Target,
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

pub struct List<K, V, T>
where
	T: Target,
	K: IndexListKey + 'static,
	V: 'static,
{
	store: ListStore<K, V, T>,
	children: HashMap<K, View<T>>,
	scope: Scope<Self, T>,
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
impl<K, V, T> Component<T> for List<K, V, T>
where
	T: Target,
	K: IndexListKey + 'static,
	V: 'static,
{
	type Props = ListProps<K, V, T>;

	fn name() -> &'static str {
		"List"
	}

	fn create(props: Self::Props, scope: Scope<Self, T>) -> Self {
		let store = ListStore::new(props);
		Self {
			store,
			scope,
			children: HashMap::new(),
		}
	}

	fn should_render(&self) -> bool {
		false
	}

	fn render(&mut self, _ctx: &Render<T>) -> Children<T> {
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
}

impl<K, V> Lifecycle<Web> for List<K, V, Web>
where
	K: IndexListKey + 'static,
	V: 'static,
{
	fn mount(&mut self, ctx: &mut WebContext) -> Result<(), JsValue> {
		if ctx.tree.is_none() {
			return Ok(());
		}

		let list = self.store.props.list.once();

		let children = ctx.tree.take();
		for ((k, _v), elem) in list.iter().zip(children.unwrap().into_iter()) {
			let child = ctx.view.render_child(elem, None)?;
			child.with_state(|state| {
				let state = state.as_mut().unwrap();
				let child_fragment = &state.fragment;
				ctx.fragment.child(child_fragment.clone());
				Ok::<(), JsValue>(())
			})?;

			self.children.insert(k.clone(), child);
		}

		Ok(())
	}

	fn after_render(&mut self, ctx: &mut crate::context::AfterRender<Web>) {
		let reaction = self
			.scope
			.reaction(move |this: &mut Self, ctx: &mut Update<Web>| {
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
}

pub struct DiffApply<'a, K, V>
where
	K: IndexListKey,
	V: 'static,
{
	view: &'a View<Web>,
	list: &'a mut List<K, V, Web>,
	next: &'a IndexList<K, V>,
}

impl<'a, K, V> DiffApply<'a, K, V>
where
	K: IndexListKey,
	V: 'static,
{
}

#[cfg(feature = "web")]
impl<'a, K, V> DiffCallback<K> for DiffApply<'a, K, V>
where
	K: IndexListKey,
	V: 'static,
{
	fn inserted(&mut self, key: &K, hint: DiffPositionHint) {
		web_sys::console::log_1(&format!("Inserted {:?}", key).into());

		let item_foo = &self.list.store.props.item;
		let element = item_foo(key, &self.next[key]);
		let child = self.list.scope.child(element, None).unwrap();

		{
			let fragment = &self.view.platform().fragment;
			let child_fragment = &child.platform().fragment;

			let idx = self.next.get_full(key).unwrap().0;

			let position = match hint {
				DiffPositionHint::Beginning => ChildPosition::Prepend,
				DiffPositionHint::End => ChildPosition::Append,
				DiffPositionHint::BeforeNext => {
					let before = self
						.next
						.get_index(idx + 1)
						.and_then(|r| self.list.children.get(r.0))
						.map(|v| v.platform().fragment.borrow().left());

					match before {
						Some(node) => ChildPosition::Before(Cow::Owned(node)),
						None => ChildPosition::Append,
					}
				}
			};

			fragment
				.borrow_mut()
				.insert_child(FragmentChild::from(child_fragment.clone()), position)
				.unwrap();
		}

		self.list.children.insert(key.clone(), child);
	}

	fn removed(&mut self, key: &K, hint: DiffPositionHint) {
		web_sys::console::log_1(&format!("Removed {:?}", key).into());
		let child = self.list.children.remove(key);
		match child {
			Some(child) => {
				let child_fragment = &child.platform().fragment;
				self.view
					.platform()
					.fragment
					.borrow_mut()
					.remove_child(&child_fragment)
					.unwrap();
			}
			None => unreachable!(),
		}
	}

	fn moved(&mut self, key: &K) {
		web_sys::console::log_1(&format!("Moved {:?}", key).into());

		let idx = self.next.get_full(key).unwrap().0;
		let before = self
			.next
			.get_index(idx + 1)
			.and_then(|r| self.list.children.get(r.0))
			.map(|v| v.platform().fragment.borrow().left());

		let child = self.list.children.get(key).unwrap();
		let child_fragment = &child.platform().fragment;

		let fragment = &self.view.platform().fragment;
		fragment
			.borrow_mut()
			.move_child(child_fragment, before)
			.unwrap();
	}
}

pub enum DiffPositionHint {
	BeforeNext,
	Beginning,
	End,
}

/// Gets notified for each step of the diffing process.
pub trait DiffCallback<K> {
	/// Called when a new element was inserted.
	fn inserted(&mut self, key: &K, position: DiffPositionHint);

	/// Called when an element was removed.
	fn removed(&mut self, key: &K, position: DiffPositionHint);

	/// Called when an element was moved.
	///
	/// The default definition reduces to calls to [`removed`] and [`inserted`].
	fn moved(&mut self, key: &K);
}

pub fn diff_with_hint<K, V, C>(
	prev: &IndexList<K, V>,
	next: &IndexList<K, V>,
	hint: ListHint,
	cb: &mut C,
) where
	K: IndexListKey,
	C: DiffCallback<K>,
{
	let next_len = next.len();
	let prev_len = prev.len();

	if hint == ListHint::Append {
		if next_len > prev_len {
			next.iter()
				.skip(prev_len)
				.for_each(|(k, _)| cb.inserted(k, DiffPositionHint::End));
			return;
		} else if next_len == prev_len {
			return;
		}
	} else if hint == ListHint::Pop {
		if next_len < prev_len {
			prev.iter()
				.skip(next_len)
				.rev()
				.for_each(|(k, _)| cb.removed(k, DiffPositionHint::End));
			return;
		} else if next_len == prev_len {
			// Nothing changed
			return;
		}
	} else if hint == ListHint::Prepend {
		if next_len > prev_len {
			next.iter()
				.take(next_len - prev_len)
				.for_each(|(k, _)| cb.inserted(k, DiffPositionHint::Beginning));
			return;
		} else if next_len == prev_len {
			return;
		}
	} else if hint == ListHint::Shift {
		if next_len < prev_len {
			next.iter()
				.take(prev_len - next_len)
				.for_each(|(k, _)| cb.removed(k, DiffPositionHint::Beginning));
			return;
		} else if next_len == prev_len {
			// Nothing changed
			return;
		}
	}

	diff(prev, next, cb)
}

fn diff<K, V, C>(prev: &IndexList<K, V>, next: &IndexList<K, V>, cb: &mut C)
where
	K: IndexListKey,
	C: DiffCallback<K>,
{
	let mut prev_iter = prev.keys().de_peekable();
	let mut next_iter = next.keys().de_peekable();

	// Sync nodes with same key at start
	while prev_iter
		.peek()
		.and_then(|kprev| next_iter.peek().filter(|knext| *kprev == **knext))
		.is_some()
	{
		prev_iter.next().unwrap();
		next_iter.next().unwrap();
	}

	// Sync nodes with same key at end
	while prev_iter
		.peek_back()
		.and_then(|kprev| next_iter.peek_back().filter(|knext| *kprev == **knext))
		.is_some()
	{
		prev_iter.next_back().unwrap();
		next_iter.next_back().unwrap();
	}

	if prev_iter.peek().is_none() {
		return next_iter
			.rev()
			.for_each(|k| cb.inserted(k, DiffPositionHint::BeforeNext)); // If all of `prev` was synced, add remaining in `next`
	} else if next_iter.peek().is_none() {
		return prev_iter.for_each(|k| cb.removed(k, DiffPositionHint::BeforeNext)); // If all of `next` was synced, remove remaining in `prev`
	}

	let mut moved: IndexSet<K> = IndexSet::new();

	for key in prev_iter {
		if next.contains_key(key) {
			moved.insert(key.clone());
		} else {
			cb.removed(key, DiffPositionHint::BeforeNext)
		}
	}

	web_sys::console::log_1(&format!("MOVED: {:?}", moved).into());

	if moved.len() > 0 {
		let lis = longest_increasing_subsequence_by(&moved, |a, b| {
			let a = prev.get_full(a).unwrap().0;
			let b = prev.get_full(b).unwrap().0;
			a.cmp(&b)
		});

		web_sys::console::log_1(&format!("LIS: {:?}", lis).into());

		let mut seq = lis.into_iter().rev().peekable();

		for key in next_iter.rev() {
			let moved = moved.get_full(key).map(|v| v.0);
			if let Some(idx) = moved {
				if let Some(true) = seq.peek().map(|v| *v == idx) {
					seq.next();
				} else {
					cb.moved(key)
				}
			} else {
				cb.inserted(key, DiffPositionHint::BeforeNext)
			}
		}
	} else {
		for key in next_iter.rev() {
			cb.inserted(key, DiffPositionHint::BeforeNext)
		}
	}
}

fn longest_increasing_subsequence_by<F, K>(a: &IndexSet<K>, mut f: F) -> Vec<usize>
where
	F: FnMut(&K, &K) -> Ordering,
	K: IndexListKey,
{
	let (mut p, mut m) = (vec![0; a.len()], Vec::with_capacity(a.len()));
	let mut it = a.iter().rev().enumerate();
	m.push(if let Some((i, _)) = it.next() {
		i
	} else {
		return Vec::new(); // The array was empty
	});

	for (i, x) in it {
		// Test whether a[i] can extend the current sequence
		if f(&a.get_index(*m.last().unwrap()).unwrap(), x) == Ordering::Less {
			p[i] = *m.last().unwrap();
			m.push(i);
			continue;
		}

		// Binary search for largest j ≤ m.len() such that a[m[j]] < a[i]
		let j =
			match m.binary_search_by(|&j| f(&a.get_index(j).unwrap(), x).then(Ordering::Greater)) {
				Ok(j) | Err(j) => j,
			};
		if j > 0 {
			p[i] = m[j - 1];
		}
		m[j] = i;
	}

	// Reconstruct the longest increasing subsequence
	let mut k = *m.last().unwrap();
	for i in (0..m.len()).rev() {
		m[i] = k;
		k = p[k];
	}
	m
}
