use super::{iter::IteratorExt, IndexList, IndexListKey, List, ListHint};
use crate::{
	web::{
		fragment::{ChildPosition, FragmentChild},
		WebElementState,
	},
	View,
};

use indexmap::IndexSet;
use std::{borrow::Cow, cmp::Ordering};

pub struct DiffApply<'a, K, V>
where
	K: IndexListKey,
	V: 'static,
{
	pub(crate) view: &'a View,
	pub(crate) list: &'a mut List<K, V>,
	pub(crate) next: &'a IndexList<K, V>,
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
		let item_foo = &self.list.store.props.item;
		let element = item_foo(key, &self.next[key]);
		let child = self.list.scope.child(element, None).unwrap();

		{
			let state = self.view.state();
			let fragment = &state.downcast_ref::<WebElementState>().unwrap().fragment;

			let child_state = child.state();
			let child_fragment = &child_state
				.downcast_ref::<WebElementState>()
				.unwrap()
				.fragment;

			let idx = self.next.get_full(key).unwrap().0;

			let position = match hint {
				DiffPositionHint::Beginning => ChildPosition::Prepend,
				DiffPositionHint::End => ChildPosition::Append,
				DiffPositionHint::BeforeNext => {
					let before = self
						.next
						.get_index(idx + 1)
						.and_then(|r| self.list.children.get(r.0))
						.map(|v| {
							v.state()
								.downcast_ref::<WebElementState>()
								.unwrap()
								.fragment
								.borrow()
								.left()
						});

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

	fn removed(&mut self, key: &K, _hint: DiffPositionHint) {
		let child = self.list.children.remove(key);
		match child {
			Some(child) => {
				let child_state = child.state();
				let child_fragment = &child_state
					.downcast_ref::<WebElementState>()
					.unwrap()
					.fragment;
				self.view
					.state()
					.downcast_ref::<WebElementState>()
					.unwrap()
					.fragment
					.borrow_mut()
					.remove_child(&child_fragment)
					.unwrap();
			}
			None => unreachable!(),
		}
	}

	fn moved(&mut self, key: &K) {
		let idx = self.next.get_full(key).unwrap().0;
		let before = self
			.next
			.get_index(idx + 1)
			.and_then(|r| self.list.children.get(r.0))
			.map(|v| {
				v.state()
					.downcast_ref::<WebElementState>()
					.unwrap()
					.fragment
					.borrow()
					.left()
			});

		let child = self.list.children.get(key).unwrap();

		let child_state = child.state();
		let child_fragment = &child_state
			.downcast_ref::<WebElementState>()
			.unwrap()
			.fragment;

		let state = self.view.state();
		let fragment = &state.downcast_ref::<WebElementState>().unwrap().fragment;

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

	if moved.len() > 0 {
		let lis = longest_increasing_subsequence_by(&moved, |a, b| {
			let a = prev.get_full(a).unwrap().0;
			let b = prev.get_full(b).unwrap().0;
			a.cmp(&b)
		});

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
