use std::{ops::Deref, rc::Rc};

use crate::{target::Target, Instance};

pub struct Handler<E, T>
where
    T: Target,
{
    pub event: E,
    pub instance: Rc<Instance<T>>,
}

impl<E, T> Deref for Handler<E, T>
where
    T: Target,
{
    type Target = Rc<Instance<T>>;
    fn deref(&self) -> &Rc<Instance<T>> {
        &self.instance
    }
}
