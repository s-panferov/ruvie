use crate::{target::Target, Component, Instance};
use snowflake::ProcessUniqueId;
use std::{
    marker::PhantomData,
    ops::Deref,
    rc::{Rc, Weak},
};

pub struct ComponentRef<C, T>
where
    C: Component<T>,
    T: Target,
{
    pub id: ProcessUniqueId,
    _c: PhantomData<C>,
    _t: PhantomData<T>,
}

impl<C, T> Deref for ComponentRef<C, T>
where
    C: Component<T>,
    T: Target,
{
    type Target = ProcessUniqueId;
    fn deref(&self) -> &ProcessUniqueId {
        &self.id
    }
}

impl<C, T> ComponentRef<C, T>
where
    C: Component<T>,
    T: Target,
{
    pub fn bind(&self, inst: &Rc<Instance<T>>) -> BoundComponentRef<C, T> {
        BoundComponentRef {
            id: fxhash::hash64(&self.id),
            parent: Rc::downgrade(inst),
            _c: PhantomData,
        }
    }
}

impl<C, T> Default for ComponentRef<C, T>
where
    C: Component<T>,
    T: Target,
{
    fn default() -> Self {
        ComponentRef {
            id: ProcessUniqueId::new(),
            _c: PhantomData,
            _t: PhantomData,
        }
    }
}

pub struct BoundComponentRef<C, T>
where
    C: Component<T>,
    T: Target,
{
    pub id: u64,
    pub parent: Weak<Instance<T>>,
    pub _c: PhantomData<C>,
}

impl<C, T> Clone for BoundComponentRef<C, T>
where
    C: Component<T>,
    T: Target,
{
    fn clone(&self) -> Self {
        BoundComponentRef {
            id: self.id.clone(),
            parent: self.parent.clone(),
            _c: PhantomData,
        }
    }
}

impl<C, T> From<BoundComponentRef<C, T>> for BoundRef<T>
where
    C: Component<T>,
    T: Target,
{
    fn from(v: BoundComponentRef<C, T>) -> Self {
        BoundRef {
            parent: v.parent,
            id: v.id,
        }
    }
}

#[derive(Clone)]
pub struct BoundRef<T: Target> {
    pub parent: Weak<Instance<T>>,
    pub id: u64,
}

impl<T> BoundRef<T>
where
    T: Target,
{
    pub fn apply(&self, inst: Weak<Instance<T>>) {
        if let Some(parent) = self.parent.upgrade() {
            parent.register_reference(&self.id, inst)
        }
    }
}

pub trait CompatibleReference<C, T>
where
    C: Component<T>,
    T: Target,
{
    fn to_bound_ref(self) -> BoundRef<T>;
}

impl<C, T> CompatibleReference<C, T> for BoundRef<T>
where
    C: Component<T>,
    T: Target,
{
    fn to_bound_ref(self) -> BoundRef<T> {
        self
    }
}

impl<C, T> CompatibleReference<C, T> for BoundComponentRef<C, T>
where
    C: Component<T>,
    T: Target,
{
    fn to_bound_ref(self) -> BoundRef<T> {
        self.into()
    }
}
