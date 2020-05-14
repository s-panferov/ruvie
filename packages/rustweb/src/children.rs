use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
};

use crate::{target::Target, Child, Component, Layout};

#[derive(Clone)]
pub struct Children<T> {
    children: Option<Vec<Rc<dyn Child<T>>>>,
}

impl<T> Children<T> {
    pub fn take(&mut self) -> Children<T> {
        self.children.take().into()
    }

    pub fn unwrap(self) -> Vec<Rc<dyn Child<T>>> {
        self.children.unwrap()
    }
}

impl<T> Deref for Children<T> {
    type Target = Option<Vec<Rc<dyn Child<T>>>>;
    fn deref(&self) -> &Self::Target {
        &self.children
    }
}

impl<T> DerefMut for Children<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.children
    }
}

impl<T> From<Option<Vec<Rc<dyn Child<T>>>>> for Children<T> {
    fn from(children: Option<Vec<Rc<dyn Child<T>>>>) -> Self {
        Children { children }
    }
}

impl<T> From<Vec<Rc<dyn Child<T>>>> for Children<T> {
    fn from(children: Vec<Rc<dyn Child<T>>>) -> Self {
        Children {
            children: Some(children),
        }
    }
}

impl<T> From<Rc<dyn Child<T>>> for Children<T> {
    fn from(children: Rc<dyn Child<T>>) -> Self {
        Children {
            children: Some(vec![children]),
        }
    }
}

impl<C: Component<T>, T> From<Layout<C, T>> for Children<T>
where
    T: Target,
{
    fn from(children: Layout<C, T>) -> Self {
        Children {
            children: Some(vec![Rc::new(children)]),
        }
    }
}
