use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
};

use crate::{AnyLayout, Component, Layout};

pub type Child<T> = Rc<dyn AnyLayout<T>>;

#[derive(Clone)]
pub struct Children<T> {
    children: Option<Vec<Child<T>>>,
}

impl<T> Children<T> {
    pub fn take(&mut self) -> Children<T> {
        self.children.take().into()
    }

    pub fn unwrap(self) -> Vec<Child<T>> {
        self.children.unwrap()
    }
}

impl<T> Deref for Children<T> {
    type Target = Option<Vec<Child<T>>>;
    fn deref(&self) -> &Self::Target {
        &self.children
    }
}

impl<T> DerefMut for Children<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.children
    }
}

impl<T> From<Option<Vec<Child<T>>>> for Children<T> {
    fn from(children: Option<Vec<Child<T>>>) -> Self {
        Children { children }
    }
}

impl<T> From<Vec<Child<T>>> for Children<T> {
    fn from(children: Vec<Child<T>>) -> Self {
        Children {
            children: Some(children),
        }
    }
}

impl<T> From<Child<T>> for Children<T> {
    fn from(children: Child<T>) -> Self {
        Children {
            children: Some(vec![children]),
        }
    }
}

impl<C: Component> From<Layout<C>> for Children<C::Target> {
    fn from(children: Layout<C>) -> Self {
        Children {
            children: Some(vec![Rc::new(children)]),
        }
    }
}
