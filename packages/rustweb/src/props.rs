use std::collections::hash_map::DefaultHasher;

use crate::Component;

use downcast_rs::{impl_downcast, Downcast};
use std::{
    any::TypeId,
    collections::HashSet,
    hash::{Hash, Hasher},
    marker::PhantomData,
};

pub trait Prop {
    type Value;
}

pub trait PropFor<C: Component>: Prop + Downcast {}

struct PropValue<P>
where
    P: Prop + Hash + 'static,
{
    prop: P,
    value: P::Value,
}

impl<P> PropValue<P>
where
    P: Prop + Hash + 'static,
{
    // fn is(&self, id: &TypeId) -> bool {}
}

impl<P> Hash for PropValue<P>
where
    P: Prop + Hash + 'static,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        TypeId::of::<P>().hash(state);
        self.prop.hash(state)
    }
}

trait ComponentPropValue<C: Component>: Downcast {
    fn prop_hash(&self) -> u64;
    // fn is(&self, id: TypeId) -> bool;
}

impl_downcast!(ComponentPropValue<C> where C: Component);

impl<C, P> ComponentPropValue<C> for PropValue<P>
where
    P: PropFor<C> + Hash + 'static,
    C: Component,
{
    fn prop_hash(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
    }
}

pub struct BoxedValue<C>
where
    C: Component,
{
    value: Box<dyn ComponentPropValue<C>>,
}

impl<C> BoxedValue<C>
where
    C: Component,
{
    pub fn downcast<P: PropFor<C> + Hash>(&self) -> Option<(&P, &P::Value)> {
        self.value
            .downcast_ref::<PropValue<P>>()
            .map(|v| (&v.prop, &v.value))
    }
}

impl<C> Hash for BoxedValue<C>
where
    C: Component,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.prop_hash().hash(state)
    }
}

impl<C> PartialEq<BoxedValue<C>> for BoxedValue<C>
where
    C: Component,
{
    fn eq(&self, other: &BoxedValue<C>) -> bool {
        self.value.prop_hash() == other.value.prop_hash()
    }
}

impl<C> Eq for BoxedValue<C> where C: Component {}

pub struct Props<C: Component> {
    _c: PhantomData<C>,
    pub props: HashSet<BoxedValue<C>>,
}

impl<C> Props<C>
where
    C: Component,
{
    pub fn new() -> Self {
        Props {
            _c: PhantomData,
            props: HashSet::new(),
        }
    }

    pub fn value_for<P: PropFor<C> + Hash>(&mut self, prop: P, value: P::Value) -> &mut Self {
        let _ = self.props.insert(BoxedValue {
            value: Box::new(PropValue { prop, value }),
        });
        self
    }
}

impl<C> Default for Props<C>
where
    C: Component,
{
    fn default() -> Self {
        Props {
            _c: PhantomData,
            props: HashSet::new(),
        }
    }
}

pub trait DynamicProps<C>
where
    C: Component,
{
}

impl<C> DynamicProps<C> for Props<C> where C: Component {}
