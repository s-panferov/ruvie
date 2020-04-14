use crate::target::Target;

use observe::local::EvalContext;

use crate::instance::Instance;
use crate::Component;

pub struct Update<'a, T: Target> {
    pub eval: &'a mut EvalContext,
    pub(crate) instance: &'a Instance<T>,
}

impl<'a, T: Target> Update<'a, T> {
    pub fn typed<C: Component<Target = T>>(self) -> UpdateT<'a, C> {
        UpdateT {
            props: self
                .instance
                .opts
                .layout
                .props()
                .downcast_ref::<C::Props>()
                .unwrap(),
            eval: self.eval,
            instance: self.instance,
        }
    }
}

pub struct UpdateT<'a, C: Component> {
    pub props: &'a C::Props,
    pub eval: &'a mut EvalContext,
    pub(crate) instance: &'a Instance<C::Target>,
}
