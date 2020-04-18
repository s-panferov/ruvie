use crate::{target::Target, Component, Instance};
use snowflake::ProcessUniqueId;
use std::{
    marker::PhantomData,
    rc::{Rc, Weak},
};

pub struct ComponentRef<C>
where
    C: Component,
{
    pub id: ProcessUniqueId,
    _t: PhantomData<C>,
}

impl<C> AsRef<ProcessUniqueId> for ComponentRef<C> where
C: Component {
    fn as_ref(&self) -> &ProcessUniqueId {
        &self.id
    }
}

impl<C> ComponentRef<C>
where
    C: Component,
{
    pub fn inside(&self, inst: &dyn AsRef<Rc<Instance<C::Target>>>) -> BoundComponentRef<C> {
        BoundComponentRef {
            id: self.id.clone(),
            parent: Rc::downgrade(inst.as_ref()),
        }
    }
}

impl<C> Default for ComponentRef<C>
where
    C: Component,
{
    fn default() -> Self {
        ComponentRef {
            id: ProcessUniqueId::new(),
            _t: PhantomData,
        }
    }
}

pub struct BoundComponentRef<C>
where
    C: Component,
{
    pub id: ProcessUniqueId,
    pub parent: Weak<Instance<C::Target>>,
}

impl<C> Clone for BoundComponentRef<C>
where
    C: Component,
{
    fn clone(&self) -> Self {
        BoundComponentRef {
            id: self.id.clone(),
            parent: self.parent.clone(),
        }
    }
}

impl<C> From<BoundComponentRef<C>> for BoundRef<C::Target>
where
    C: Component,
{
    fn from(v: BoundComponentRef<C>) -> Self {
        BoundRef {
            parent: v.parent,
            id: v.id,
        }
    }
}

pub struct BoundRef<T: Target> {
    pub parent: Weak<Instance<T>>,
    pub id: ProcessUniqueId,
}

impl<T> BoundRef<T>
where
    T: Target,
{
    pub fn apply(&self, inst: Weak<Instance<T>>) {
        if let Some(parent) = self.parent.upgrade() {
            parent.register_reference(self.id, inst)
        }
    }
}
