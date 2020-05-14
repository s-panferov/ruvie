use std::{marker::PhantomData, rc::Rc};

use crate::children::Children;
use crate::{context::Render, target::Target, Component, Layout};

pub trait FunctionalComponent<F: Fn(&mut Render<P, T>) -> Children<T>, P, T: Target> {
    fn create(self) -> Func<F, P, T>;
}

impl<F, P, T: Target> FunctionalComponent<F, P, T> for F
where
    F: Fn(&mut Render<P, T>) -> Children<T>,
{
    fn create(self) -> Func<F, P, T> {
        Func {
            func: self,
            p: PhantomData,
            t: PhantomData,
        }
    }
}

pub struct Func<F: Fn(&mut Render<P, T>) -> Children<T>, P, T: Target> {
    func: F,
    p: PhantomData<P>,
    t: PhantomData<T>,
}

impl<F: Fn(&mut Render<P, T>) -> Children<T>, P, T: Target> Func<F, P, T> {
    pub fn new(func: F) -> Self {
        Func {
            func,
            p: PhantomData,
            t: PhantomData,
        }
    }
}

impl<F, P, T: Target> Component<T> for Func<F, P, T>
where
    F: Fn(&mut Render<P, T>) -> Children<T>,
    F: 'static,
    P: 'static,
    T: 'static,
{
    type Props = P;

    fn render(&self, ctx: &mut Render<P, T>) -> Children<T> {
        (self.func)(ctx)
    }
}

impl<F, T: Target> From<F> for Layout<Func<F, (), T>, T>
where
    F: Fn(&mut Render<(), T>) -> Children<T>,
    F: 'static,
{
    fn from(func: F) -> Self {
        Layout {
            reference: None,
            component: Func::new(func),
            props: Rc::new(()),
            children: None.into(),
        }
    }
}
