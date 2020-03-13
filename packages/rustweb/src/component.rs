use observe::EvalContext;

use crate::dom::Html;
use crate::instance::Instance;
use crate::layout::Children;
use crate::{layout::Layout, scheduler::WeakScheduler};

use std::marker::PhantomData;

pub struct Context<'a, P = (), T: Target = Html> {
    pub props: &'a P,
    pub eval: &'a mut EvalContext,
    pub children: &'a Children<T>,
}

pub struct UpdateContext<'a, P = (), T: Target = Html> {
    pub props: &'a P,
    pub eval: &'a mut EvalContext,
    pub(crate) instance: &'a Instance<T>,
}

pub trait Target: Clone + 'static {
    type MountContext;
    type Error;
    type Runtime;
    type Result;

    fn component(
        ctx: &mut Self::MountContext,
        tree: Children<Self>,
    ) -> Result<Self::Result, Self::Error>;

    fn mount(mount: &mut Instance<Self>) -> Result<Self::Result, Self::Error>;
    fn tick(scheduler: WeakScheduler<Self>);
}

pub trait Component: 'static {
    type Props;
    type Target: Target;

    fn name(&self) -> &'static str {
        return "Component";
    }

    fn render(&self, _ctx: Context<Self::Props, Self::Target>) -> Children<Self::Target> {
        _ctx.children.clone()
    }

    fn mount(
        &self,
        ctx: &mut <Self::Target as Target>::MountContext,
        tree: Children<Self::Target>,
    ) -> Result<<Self::Target as Target>::Result, <Self::Target as Target>::Error> {
        Self::Target::component(ctx, tree)
    }

    fn update(
        &self,
        _ctx: UpdateContext<Self::Props, Self::Target>,
    ) -> Result<(), <Self::Target as Target>::Error> {
        Ok(())
    }
}

pub trait FunctionalComponent<F: Fn(Context<P, T>) -> Children<T>, P, T: Target> {
    fn create(self) -> Func<F, P, T>;
}

impl<F, P, T: Target> FunctionalComponent<F, P, T> for F
where
    F: Fn(Context<P, T>) -> Children<T>,
{
    fn create(self) -> Func<F, P, T> {
        Func {
            func: self,
            p: PhantomData,
            t: PhantomData,
        }
    }
}

pub struct Func<F: Fn(Context<P, T>) -> Children<T>, P, T: Target> {
    func: F,
    p: PhantomData<P>,
    t: PhantomData<T>,
}

impl<F: Fn(Context<P, T>) -> Children<T>, P, T: Target> Func<F, P, T> {
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
    F: Fn(Context<P, T>) -> Children<T>,
    F: 'static,
    P: 'static,
    T: 'static,
{
    type Props = P;
    type Target = T;
    fn render(&self, ctx: Context<P, T>) -> Children<T> {
        (self.func)(ctx)
    }
}

impl<F, T: Target> From<F> for Layout<Func<F, (), T>>
where
    F: Fn(Context<(), T>) -> Children<T>,
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
