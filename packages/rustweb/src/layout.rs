use std::{any::Any, rc::Rc};

use observe::local::EvalContext;

use crate::component::Component;
use crate::reference::BoundRef;
use crate::target::Target;
use crate::{
    children::Children,
    context::{AfterRender, Render},
    Instance,
};

pub struct Layout<C: Component<T>, T>
where
    T: Target,
{
    pub(crate) component: C,
    pub(crate) props: Rc<C::Props>,
    pub(crate) children: Children<T>,
    pub(crate) reference: Option<BoundRef<T>>,
}

impl<C: Component<T>, T> Layout<C, T> where T: Target {}

impl<C, T> From<Layout<C, T>> for Box<dyn Child<T>>
where
    C: Component<T>,
    T: Target,
{
    fn from(value: Layout<C, T>) -> Self {
        Box::new(value)
    }
}

impl<C, T> From<C> for Layout<C, T>
where
    C: Component<T, Props = ()>,
    T: Target,
{
    fn from(component: C) -> Self {
        Layout {
            component,
            props: Rc::new(()),
            reference: None,
            children: None.into(),
        }
    }
}

/// This is basically a layout with an erased component type
pub trait Child<T: Target> {
    fn props(&self) -> Rc<dyn Any>;
    fn component(&self) -> &dyn Any;
    fn name(&self) -> &'static str;

    fn mount(&self, ctx: &mut T::Mount) -> Result<T::Result, T::Error>;
    fn should_render(&self) -> bool;
    fn render(&self, instance: Rc<Instance<T>>, ev: &mut EvalContext) -> Children<T>;
    fn after_render(&self, ctx: &mut AfterRender<T>);
    fn before_unmount(&self);
    fn get_ref(&self) -> Option<BoundRef<T>>;
    fn inject(&self, func: Box<dyn FnOnce(&dyn Any, &dyn Any)>);
}

impl<C, T> Child<T> for Layout<C, T>
where
    C: Component<T>,
    T: Target,
{
    fn props(&self) -> Rc<dyn Any> {
        self.props.clone()
    }

    fn component(&self) -> &dyn Any {
        &self.component
    }

    fn name(&self) -> &'static str {
        self.component.name()
    }

    fn mount(&self, ctx: &mut T::Mount) -> Result<T::Result, T::Error> {
        self.component.mount(ctx)
    }

    fn should_render(&self) -> bool {
        self.component.should_render(&self.props)
    }

    fn render(&self, instance: Rc<Instance<T>>, eval: &mut EvalContext) -> Children<T> {
        self.component.render(&mut Render {
            eval,
            children: &self.children,
            props: &self.props,
            instance,
        })
    }

    fn after_render(&self, ctx: &mut AfterRender<T>) {
        self.component.after_render(ctx)
    }

    fn before_unmount(&self) {
        self.component.before_unmount()
    }

    fn get_ref(&self) -> Option<BoundRef<T>> {
        self.reference.as_ref().map(|r| (*r).clone().into())
    }

    fn inject(&self, func: Box<dyn FnOnce(&dyn Any, &dyn Any)>) {
        (func)(&self.component, &self.props)
    }
}
