use std::{any::Any, rc::Rc};

use observe::local::EvalContext;

use crate::component::{Component, Render};
use crate::reference::{BoundComponentRef, BoundRef};
use crate::target::Target;
use crate::{after::AfterRender, children::Children, mount::Mount, Func, Instance};

pub struct Layout<C: Component> {
    pub(crate) component: C,
    pub(crate) props: C::Props,
    pub(crate) children: Children<C::Target>,
    pub(crate) reference: Option<BoundComponentRef<C>>,
}

impl<C: Component> Layout<C> {
    pub fn with_ref(mut self, refr: BoundComponentRef<C>) -> Self {
        self.reference = Some(refr);
        self
    }

    pub fn child<F, CH>(mut self, child: F) -> Self
    where
        F: Into<Layout<CH>>,
        CH: Component<Target = C::Target> + 'static,
    {
        match self.children.as_mut() {
            Some(children) => children.push(Rc::new(child.into())),
            None => self.children = child.into().into(),
        }

        self
    }

    pub fn scope<F, CH>(mut self, child: F) -> Self
    where
        F: Fn(Render<(), C::Target>) -> CH + 'static,
        CH: Into<Children<C::Target>>,
    {
        let instance = Func::new(move |ctx| child(ctx).into());
        match self.children.as_mut() {
            Some(children) => children.push(Rc::new(instance.default())),
            None => self.children = instance.default().into(),
        }

        self
    }

    pub fn children(mut self, mut children: Children<C::Target>) -> Self {
        if children.is_none() {
            return self;
        }

        match self.children.as_mut() {
            Some(current) => current.append(children.as_mut().unwrap()),
            None => self.children = children,
        }

        self
    }
}

impl<C: Component> From<Layout<C>> for Box<dyn AnyLayout<C::Target>> {
    fn from(value: Layout<C>) -> Self {
        Box::new(value)
    }
}

impl<C: Component<Props = ()>> From<C> for Layout<C> {
    fn from(component: C) -> Self {
        Layout {
            component,
            props: (),
            reference: None,
            children: None.into(),
        }
    }
}

pub trait AnyLayout<T: Target> {
    fn props(&self) -> &dyn Any;
    fn mount(&self, ctx: &mut Mount<T>) -> Result<T::Result, T::Error>;
    fn render(&self, instance: Rc<Instance<T>>, ev: &mut EvalContext) -> Children<T>;
    fn after_render(&self, ctx: &mut AfterRender<T>);
    fn before_unmount(&self);
    fn get_ref(&self) -> Option<BoundRef<T>>;
}

impl<C: Component> AnyLayout<C::Target> for Layout<C> {
    fn props(&self) -> &dyn Any {
        &self.props
    }

    fn mount(
        &self,
        ctx: &mut Mount<C::Target>,
    ) -> Result<<C::Target as Target>::Result, <C::Target as Target>::Error> {
        self.component.mount(ctx)
    }

    fn render(
        &self,
        instance: Rc<Instance<C::Target>>,
        eval: &mut EvalContext,
    ) -> Children<C::Target> {
        self.component.render(Render {
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
}
