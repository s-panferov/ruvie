use observe::local::EvalContext;

use crate::children::Children;
use crate::{
    after::AfterRender, dom::Html, instance::Instance, mount::Mount, target::Target, Layout,
};
use std::{ops::Deref, rc::Rc};

pub type RenderSelf<'a, C> = Render<'a, <C as Component>::Props, <C as Component>::Target>;

pub struct Render<'a, P = (), T: Target = Html> {
    pub props: &'a P,
    pub eval: &'a mut EvalContext,
    pub children: &'a Children<T>,
    pub instance: Rc<Instance<T>>,
}

impl<'a, P, T: Target> Deref for Render<'a, P, T> {
    type Target = Rc<Instance<T>>;
    fn deref(&self) -> &Self::Target {
        &self.instance
    }
}

impl<'a, P, T: Target> AsRef<Rc<Instance<T>>> for Render<'a, P, T> {
    fn as_ref(&self) -> &Rc<Instance<T>> {
        &self.instance
    }
}

pub trait Component: 'static + Sized {
    type Props;
    type Target: Target;

    fn name(&self) -> &'static str {
        return "Component";
    }

    fn render(&self, _ctx: Render<Self::Props, Self::Target>) -> Children<Self::Target> {
        _ctx.children.clone()
    }

    fn after_render(&self, _ctx: &mut AfterRender<Self::Target>) {}

    fn before_unmount(&self) {}

    fn mount(
        &self,
        ctx: &mut Mount<Self::Target>,
    ) -> Result<<Self::Target as Target>::Result, <Self::Target as Target>::Error> {
        Self::Target::mount_component(ctx)
    }

    fn with_props(self, props: Self::Props) -> Layout<Self> {
        Layout {
            reference: None,
            component: self,
            props,
            children: None.into(),
        }
    }

    fn default(self) -> Layout<Self>
    where
        Self::Props: Default,
    {
        Layout {
            reference: None,
            component: self,
            props: Default::default(),
            children: None.into(),
        }
    }
}
