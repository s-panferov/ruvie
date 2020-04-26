use std::{ops::Deref, rc::Rc};

use observe::local::EvalContext;

use crate::children::Children;
use crate::{dom::Html, instance::Instance, target::Target};

pub struct Render<'a, P, T: Target = Html> {
    pub props: &'a Rc<P>,
    pub eval: &'a mut EvalContext,
    pub children: &'a Children<T>,
    pub(crate) instance: Rc<Instance<T>>,
}

impl<'a, P, T> Render<'a, P, T> where T: Target {}

impl<'a, P, T: Target> Deref for Render<'a, P, T> {
    type Target = Rc<Instance<T>>;
    fn deref(&self) -> &Self::Target {
        &self.instance
    }
}
