use crate::instance::Instance;
use crate::target::Target;
use std::{ops::Deref, rc::Rc};

pub struct AfterRender<T: Target> {
    pub(crate) instance: Rc<Instance<T>>,
}

impl<'a, T> AfterRender<T> where T: Target {}

impl<'a, T> Deref for AfterRender<T>
where
    T: Target,
{
    type Target = Rc<Instance<T>>;
    fn deref(&self) -> &Self::Target {
        &self.instance
    }
}
