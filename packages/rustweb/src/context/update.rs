use std::{ops::Deref, rc::Rc};

use observe::local::EvalContext;

use crate::instance::Instance;
use crate::target::Target;

pub struct Update<'a, T: Target> {
    pub eval: &'a mut EvalContext,
    pub(crate) instance: Rc<Instance<T>>,
}

impl<'a, T> Deref for Update<'a, T>
where
    T: Target,
{
    type Target = Rc<Instance<T>>;
    fn deref(&self) -> &Self::Target {
        &self.instance
    }
}
