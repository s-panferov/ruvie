use crate::{
    context::Render, prelude::*, reference::BoundRef, target::Target, Child, Children, Component,
    Instance,
};

use std::{cell::RefCell, hash::Hash, marker::PhantomData, rc::Rc};

use observe::local::Value;

pub mod diff;

use diff::DiffCallback;

pub struct ListProps<V, K, T>
where
    T: Target,
    K: Eq + Ord,
    V: Hash,
{
    pub hint: ListHint,
    pub list: Value<Vec<V>>,
    pub key: Box<dyn Fn(&V) -> &K>,
    pub item: Box<dyn Fn(&V, BoundRef<T>) -> Rc<dyn Child<T>>>,
    pub _t: PhantomData<T>,
}

pub struct ListStore<V, K, T>
where
    T: Target,
    K: Eq + Ord + Clone,
    V: Hash,
{
    props: ListProps<V, K, T>,
    prev: RefCell<Rc<Vec<V>>>,
}

impl<V, K, T> ListStore<V, K, T>
where
    T: Target,
    K: Eq + Ord + Clone,
    V: Hash,
{
    pub fn new(props: ListProps<V, K, T>) -> Self {
        Self {
            props,
            prev: RefCell::new(Rc::new(vec![])),
        }
    }
}

pub struct List<V, K, T>
where
    T: Target,
    K: Eq,
    V: Hash,
{
    _v: PhantomData<V>,
    _k: PhantomData<K>,
    _t: PhantomData<T>,
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

impl<V, K, T> List<V, K, T>
where
    K: Eq + Ord + Clone + 'static,
    V: Hash + 'static,
    T: Target,
{
    pub fn new() -> Self {
        Self {
            _v: PhantomData,
            _k: PhantomData,
            _t: PhantomData,
        }
    }
}

#[cfg(feature = "dom")]
use crate::dom::Html;

#[cfg(feature = "dom")]
impl<V, K> Component<Html> for List<V, K, Html>
where
    K: Eq + Ord + Clone + Hash + 'static,
    V: Hash + 'static,
{
    type Props = ListStore<V, K, Html>;

    fn render(&self, ctx: &mut Render<Self::Props, Html>) -> Children<Html> {
        let store = Self::props(ctx);

        let mut children = vec![];
        let list = store.props.list.once();

        for child in list.iter() {
            let key = (store.props.key)(child);
            let refr = ctx.reference_any(&key);
            let item = (store.props.item)(child, refr);
            children.push(item)
        }

        *store.prev.borrow_mut() = list;
        children.into()
    }

    fn after_render(&self, ctx: &mut crate::context::AfterRender<Html>) {
        let store = Self::props(ctx);
        ctx.reaction(Self::reaction(move |_, ctx| {
            let mut prev = store.prev.borrow_mut();
            let next = store.props.list.observe(ctx.eval);
            let hint = store.props.hint.clone();
            let mut diff_cb = DiffApply::<V, K, Html> {
                _v: PhantomData,
                store: &store,
                instance: &ctx,
            };

            diff(&prev, &next, hint, &store.props.key, &mut diff_cb);
            *prev = next;

            // TODO we need to create a struct to be a callback for the diff operation

            Ok(())
        }));
    }
}

pub struct DiffApply<'a, V, K, T>
where
    T: Target,
    V: Hash,
    K: Hash + Ord + Clone,
{
    _v: PhantomData<V>,
    store: &'a ListStore<V, K, T>,
    instance: &'a Rc<Instance<T>>,
}

#[cfg(feature = "dom")]
impl<'a, V, K> DiffCallback<(usize, &V), (usize, &V)> for DiffApply<'a, V, K, Html>
where
    V: Hash,
    K: Hash + Ord + Clone,
{
    fn inserted(&mut self, (idx, v): (usize, &V)) {
        let refr = self.instance.reference_any((self.store.props.key)(v));
        let (dom_node, child) = self
            .instance
            .render_child((self.store.props.item)(v, refr))
            .unwrap();

        let mut state = self.instance.state_mut();
        state.children.insert(idx, child);

        let prev_child = &mut state.children.get(idx - 1);

        if let Some(prev_child) = prev_child {
            let mut prev_state = prev_child.state_mut();
            let target = prev_state.target.as_mut().unwrap();
            let prev_node = target.nodes.first().unwrap();
            before_node
                .parent_element()
                .unwrap()
                .insert_before(&dom_node, Some(&before_node))
                .unwrap();
        } else {
            let target_state = state.target.as_mut().unwrap();
            let first_node = &target_state.nodes[0];
            first_node
                .parent_element()
                .unwrap()
                .insert_before(&dom_node, None)
                .unwrap();
        }
    }

    fn removed(&mut self, old: (usize, &V)) {}
    fn unchanged(&mut self, old: (usize, &V), new: (usize, &V)) {}
    fn moved(&mut self, old: (usize, &V), new: (usize, &V)) {}
}

pub fn diff<'a, V, K, C>(
    prev: &'a [V],
    next: &'a [V],
    hint: ListHint,
    key: &dyn for<'b> Fn(&'b V) -> &'b K,
    cb: &mut C,
) where
    K: Eq + PartialEq + Hash + Clone,
    V: 'static,
    C: DiffCallback<(usize, &'a V), (usize, &'a V)>,
{
    let next_len = next.len();
    let prev_len = prev.len();

    if hint == ListHint::Append {
        if next_len > prev_len {
            let new_elements = &next[(prev_len - 1)..];
            new_elements
                .iter()
                .enumerate()
                .for_each(|(idx, el)| cb.inserted((idx, el)));
            return;
        } else if next_len == prev_len {
            return;
        }
    } else if hint == ListHint::Pop {
        if next_len < prev_len {
            let removed_elements = &prev[(next_len - 1)..];
            removed_elements
                .iter()
                .enumerate()
                .for_each(|(idx, el)| cb.removed((next_len + idx, el)));
            return;
        } else if next_len == prev_len {
            // Nothing changed
            return;
        }
    } else if hint == ListHint::Prepend {
        if next_len > prev_len {
            let new_elements = &next[0..(next_len - prev_len)];
            new_elements
                .iter()
                .enumerate()
                .for_each(|(idx, el)| cb.inserted((idx, el)));
            return;
        } else if next_len == prev_len {
            return;
        }
    } else if hint == ListHint::Shift {
        if next_len < prev_len {
            let removed_elements = &next[0..(prev_len - next_len)];
            removed_elements
                .iter()
                .enumerate()
                .for_each(|(idx, el)| cb.removed((idx, el)));
            return;
        } else if next_len == prev_len {
            // Nothing changed
            return;
        }
    }

    diff::diff_by_key(
        prev.iter().enumerate(),
        |x| key(&x.1),
        next.iter().enumerate(),
        |x| key(&x.1),
        cb,
    );
}
