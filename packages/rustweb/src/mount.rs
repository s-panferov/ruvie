use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
    rc::{Rc, Weak},
};

use observe::Tracker;

use crate::component::Component;
use crate::instance::Instance;
use crate::target::Target;
use crate::{
    update::{UpdateReaction, UpdateT},
    Children,
};

pub struct Mount<T: Target> {
    pub(crate) children: Vec<Rc<Instance<T>>>,
    pub(crate) reactions: Reactions<T>,
    pub(crate) instance: Rc<Instance<T>>,
    pub(crate) platform: <T as Target>::Mount,
    pub(crate) tree: Children<T>,
}

impl<T> Deref for Mount<T>
where
    T: Target,
{
    type Target = <T as Target>::Mount;

    fn deref(&self) -> &Self::Target {
        &self.platform
    }
}

impl<T> DerefMut for Mount<T>
where
    T: Target,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.platform
    }
}

pub struct Reactions<T: Target>
where
    T: Target,
{
    pub(crate) reactions: Vec<Tracker>,
    pub(crate) instance: Weak<Instance<T>>,
    _t: PhantomData<T>,
}

impl<T> Reactions<T>
where
    T: Target,
{
    pub fn new(instance: Weak<Instance<T>>) -> Reactions<T> {
        Reactions {
            reactions: Vec::new(),
            instance,
            _t: PhantomData,
        }
    }

    pub(crate) fn add<F, C: Component<Target = T>>(&mut self, _: &C, handler: F)
    where
        F: Fn(UpdateT<C>) -> Result<(), <C::Target as Target>::Error>,
        F: 'static,
    {
        let rx = Tracker::new("Update Reaction".to_owned());
        rx.set_computation(Box::new(UpdateReaction {
            handler,
            instance: self.instance.clone(),
            rx: rx.weak(),
        }));
        rx.autorun();
        self.reactions.push(rx)
    }
}

impl<T> Mount<T>
where
    T: Target,
{
    pub(crate) fn add_child(&mut self, child: Rc<Instance<T>>) {
        self.children.push(child);
    }
}
