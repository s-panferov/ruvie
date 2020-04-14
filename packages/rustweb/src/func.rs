use crate::children::Children;
use crate::{target::Target, Component, Layout, Render};

use std::marker::PhantomData;

pub trait FunctionalComponent<F: Fn(Render<P, T>) -> Children<T>, P, T: Target> {
    fn create(self) -> Func<F, P, T>;
}

impl<F, P, T: Target> FunctionalComponent<F, P, T> for F
where
    F: Fn(Render<P, T>) -> Children<T>,
{
    fn create(self) -> Func<F, P, T> {
        Func {
            func: self,
            p: PhantomData,
            t: PhantomData,
        }
    }
}

pub struct Func<F: Fn(Render<P, T>) -> Children<T>, P, T: Target> {
    func: F,
    p: PhantomData<P>,
    t: PhantomData<T>,
}

impl<F: Fn(Render<P, T>) -> Children<T>, P, T: Target> Func<F, P, T> {
    pub fn new(func: F) -> Self {
        Func {
            func,
            p: PhantomData,
            t: PhantomData,
        }
    }
}

impl<F, P, T: Target> Component for Func<F, P, T>
where
    F: Fn(Render<P, T>) -> Children<T>,
    F: 'static,
    P: 'static,
    T: 'static,
{
    type Props = P;
    type Target = T;
    fn render(&self, ctx: Render<P, T>) -> Children<T> {
        (self.func)(ctx)
    }
}

impl<F, T: Target> From<F> for Layout<Func<F, (), T>>
where
    F: Fn(Render<(), T>) -> Children<T>,
    F: 'static,
{
    fn from(func: F) -> Self {
        Layout {
            component: Func::new(func),
            props: (),
            children: None.into(),
        }
    }
}
