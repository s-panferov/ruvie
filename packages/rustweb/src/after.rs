use crate::instance::Instance;
use crate::mount::Reactions;
use crate::target::Target;
use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
};

pub struct AfterRender<T: Target> {
    pub(crate) instance: Rc<Instance<T>>,
    pub reactions: Reactions<T>,
}

impl<'a, T> AfterRender<T> where T: Target {}

impl<'a, T> AsRef<Instance<T>> for AfterRender<T>
where
    T: Target,
{
    fn as_ref(&self) -> &Instance<T> {
        &self.instance
    }
}

impl<'a, T> Deref for AfterRender<T>
where
    T: Target,
{
    type Target = Rc<Instance<T>>;
    fn deref(&self) -> &Self::Target {
        &self.instance
    }
}

impl<'a, T> DerefMut for AfterRender<T>
where
    T: Target,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.instance
    }
}
