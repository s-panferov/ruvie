use crate::{
    context::Render, dom::Html, prelude::*, target::Target, Child, Children, Component, Instance,
};
use std::{cell::RefCell, hash::Hash, marker::PhantomData, rc::Rc};

use lis::DiffCallback;
use observe::local::Value;

pub struct ListProps<V, K, T>
where
    T: Target,
    K: Eq + Ord,
    V: Hash,
{
    pub hint: ListHint,
    pub list: Value<Vec<V>>,
    pub key: Box<dyn Fn(&V) -> &K>,
    pub item: Box<dyn Fn(&V) -> Rc<dyn Child<T>>>,
    _t: PhantomData<T>,
}

pub struct ListStore<V, K, T>
where
    T: Target,
    K: Eq + Ord + Clone,
    V: Hash,
{
    props: ListProps<V, K, T>,
    prev: RefCell<Rc<Vec<Rc<V>>>>,
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

trait TypeEquals {
    type Other;
}

impl<V, K, T> Component for List<V, K, T>
where
    T: Target,
    K: Eq + Ord + Clone + 'static,
    V: Hash,
    V: 'static,
{
    type Props = ListStore<V, K, T>;
    type Target = T;

    fn render(&self, ctx: &mut Render<Self::Props, Self::Target>) -> Children<Self::Target> {
        let store = Self::props(ctx);

        let mut children = vec![];
        let list = store.props.list.observe(ctx.eval);
        let hint = store.props.hint.clone();

        for child in list.iter() {
            let item = (store.props.item)(child);
            children.push(item)
        }

        children.into()
    }

    fn after_render(&self, ctx: &mut crate::context::AfterRender<Html>) {
        let store = Self::props(ctx);
        ctx.reaction(Self::reaction(move |_, ctx| {
            let prev = store.prev.borrow_mut();
            let next = store.props.list.observe(ctx.eval);
            let node = crate::dom::node(&ctx);
            Ok(())
        }));
    }
}

// pub struct DiffApply<V, V> {
//     _v: PhantomData<V>,
//     instance: Rc<Instance<T>>,
// }

// impl<V> DiffCallback<(usize, &V), (usize, &V)> for DiffApply<V> {
//     fn inserted(&mut self, new: (usize, &V)) {}
//     fn removed(&mut self, old: (usize, &V)) {}
//     fn unchanged(&mut self, old: (usize, &V), new: (usize, &V)) {}
//     fn moved(&mut self, old: (usize, &V), new: (usize, &V)) {}
// }

// pub fn diff<'a, V, K, C>(
//     prev: &'a [V],
//     next: &'a [V],
//     hint: ListHint,
//     key: &dyn for<'b> Fn(&'b V) -> &'b K,
//     cb: &mut C,
// ) where
//     K: Eq + PartialEq + Hash + Clone,
//     V: 'static,
//     C: DiffCallback<(usize, &'a V), (usize, &'a V)>,
// {
//     let next_len = next.len();
//     let prev_len = prev.len();

//     if hint == ListHint::Append {
//         if next_len > prev_len {
//             let new_elements = &next[(prev_len - 1)..];
//             new_elements
//                 .iter()
//                 .enumerate()
//                 .for_each(|(idx, el)| cb.inserted((idx, el)));
//             return;
//         } else if next_len == prev_len {
//             return;
//         }
//     } else if hint == ListHint::Pop {
//         if next_len < prev_len {
//             let removed_elements = &prev[(next_len - 1)..];
//             removed_elements
//                 .iter()
//                 .enumerate()
//                 .for_each(|(idx, el)| cb.removed((next_len + idx, el)));
//             return;
//         } else if next_len == prev_len {
//             // Nothing changed
//             return;
//         }
//     } else if hint == ListHint::Prepend {
//         if next_len > prev_len {
//             let new_elements = &next[0..(next_len - prev_len)];
//             new_elements
//                 .iter()
//                 .enumerate()
//                 .for_each(|(idx, el)| cb.inserted((idx, el)));
//             return;
//         } else if next_len == prev_len {
//             return;
//         }
//     } else if hint == ListHint::Shift {
//         if next_len < prev_len {
//             let removed_elements = &next[0..(prev_len - next_len)];
//             removed_elements
//                 .iter()
//                 .enumerate()
//                 .for_each(|(idx, el)| cb.removed((idx, el)));
//             return;
//         } else if next_len == prev_len {
//             // Nothing changed
//             return;
//         }
//     }

//     lis::diff_by_key(
//         prev.iter().enumerate(),
//         |x| key(&x.1),
//         next.iter().enumerate(),
//         |x| key(&x.1),
//         cb,
//     );
// }

// // #[cfg(test)]
// // mod tests {
// //     use super::{diff, ListHint};
// // }
