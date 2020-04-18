use std::rc::{Rc, Weak};

use observe::local::EvalContext;
use observe::{Evaluation, Local, WeakTracker};

use crate::instance::Instance;
use crate::target::Target;
use crate::Component;

pub struct Update<'a, T: Target> {
    pub eval: &'a mut EvalContext,
    pub(crate) instance: &'a Rc<Instance<T>>,
}

impl<'a, T> AsRef<Instance<T>> for Update<'a, T>
where
    T: Target,
{
    fn as_ref(&self) -> &Instance<T> {
        &self.instance
    }
}

impl<'a, T: Target> Update<'a, T> {
    pub fn typed<C: Component<Target = T>>(self) -> UpdateT<'a, C> {
        let props = self
            .instance
            .spec
            .layout
            .props()
            .downcast_ref::<C::Props>()
            .unwrap();

        UpdateT {
            props,
            eval: self.eval,
            instance: self.instance,
        }
    }
}

pub struct UpdateT<'a, C: Component> {
    pub props: &'a C::Props,
    pub eval: &'a mut EvalContext,
    pub(crate) instance: &'a Rc<Instance<C::Target>>,
}

pub struct UpdateReaction<C, F>
where
    C: Component,
    F: Fn(UpdateT<C>) -> Result<(), <C::Target as Target>::Error>,
{
    pub instance: Weak<Instance<C::Target>>,
    pub rx: WeakTracker<Local>,
    pub handler: F,
}

impl<C, F> Evaluation<Local> for UpdateReaction<C, F>
where
    C: Component,
    F: Fn(UpdateT<C>) -> Result<(), <C::Target as Target>::Error>,
{
    fn is_scheduled(&self) -> bool {
        true
    }

    fn evaluate(&mut self, eval: &mut EvalContext) -> u64 {
        if let Some(instance) = self.instance.upgrade() {
            let ctx = Update {
                eval,
                instance: &instance,
            }
            .typed::<C>();
            let res = (self.handler)(ctx);
            instance.state_mut().update_res = res;
        } else {
            unreachable!()
        }
        0
    }

    fn on_reaction(&mut self) {
        if let Some(instance) = self.instance.upgrade() {
            // FIXME move to instance
            instance
                .state_mut()
                .invalidated_updates
                .insert(self.rx.clone());
            // FIXME triggers several times, need to optimize in observe
            instance.schedule_update()
        } else {
            unreachable!()
        }
    }
}
