use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use crate::instance::Instance;
use crate::target::Target;
use crate::Children;

pub struct Mount<T: Target> {
    pub(crate) children: Vec<Rc<Instance<T>>>,
    pub(crate) tree: Children<T>,
    pub(crate) instance: Rc<Instance<T>>,
}

impl<T> Mount<T>
where
    T: Target,
{
    pub(crate) fn add_child(&mut self, child: Rc<Instance<T>>) {
        self.children.push(child);
    }
}

impl<T> Deref for Mount<T>
where
    T: Target,
{
    type Target = Rc<Instance<T>>;
    fn deref(&self) -> &Rc<Instance<T>> {
        &self.instance
    }
}
