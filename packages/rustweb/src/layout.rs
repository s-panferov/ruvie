use std::{any::Any, rc::Rc};

use observe::local::EvalContext;

use crate::component::Component;
use crate::reference::{BoundComponentRef, BoundRef};
use crate::target::Target;
use crate::{
    children::Children,
    context::{AfterRender, Render},
    Instance,
};

pub struct Layout<C: Component> {
    pub(crate) component: C,
    pub(crate) props: Rc<C::Props>,
    pub(crate) children: Children<C::Target>,
    pub(crate) reference: Option<BoundComponentRef<C>>,
}

impl<C: Component> Layout<C> {}

impl<C: Component> From<Layout<C>> for Box<dyn Child<C::Target>> {
    fn from(value: Layout<C>) -> Self {
        Box::new(value)
    }
}

impl<C: Component<Props = ()>> From<C> for Layout<C> {
    fn from(component: C) -> Self {
        Layout {
            component,
            props: Rc::new(()),
            reference: None,
            children: None.into(),
        }
    }
}

pub trait Child<T: Target> {
    fn props(&self) -> Rc<dyn Any>;
    fn component(&self) -> &dyn Any;
    fn name(&self) -> &'static str;

    fn mount(&self, ctx: &mut T::Mount) -> Result<T::Result, T::Error>;
    fn render(&self, instance: Rc<Instance<T>>, ev: &mut EvalContext) -> Children<T>;
    fn after_render(&self, ctx: &mut AfterRender<T>);
    fn before_unmount(&self);
    fn get_ref(&self) -> Option<BoundRef<T>>;
    fn inject(&self, func: Box<dyn FnOnce(&dyn Any, &dyn Any)>);
}

impl<C: Component> Child<C::Target> for Layout<C> {
    fn props(&self) -> Rc<dyn Any> {
        self.props.clone()
    }

    fn component(&self) -> &dyn Any {
        &self.component
    }

    fn name(&self) -> &'static str {
        self.component.name()
    }

    fn mount(
        &self,
        ctx: &mut <C::Target as Target>::Mount,
    ) -> Result<<C::Target as Target>::Result, <C::Target as Target>::Error> {
        self.component.mount(ctx)
    }

    fn render(
        &self,
        instance: Rc<Instance<C::Target>>,
        eval: &mut EvalContext,
    ) -> Children<C::Target> {
        self.component.render(&mut Render {
            eval,
            children: &self.children,
            props: &self.props,
            instance,
        })
    }

    fn after_render(&self, ctx: &mut AfterRender<C::Target>) {
        self.component.after_render(ctx)
    }

    fn before_unmount(&self) {
        self.component.before_unmount()
    }

    fn get_ref(&self) -> Option<BoundRef<C::Target>> {
        self.reference.as_ref().map(|r| (*r).clone().into())
    }

    fn inject(&self, func: Box<dyn FnOnce(&dyn Any, &dyn Any)>) {
        (func)(&self.component, &self.props)
    }
}
