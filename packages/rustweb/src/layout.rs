use std::{any::Any, rc::Rc};

use observe::local::EvalContext;

use crate::component::{Component, Render};
use crate::target::Target;
use crate::{children::Children, Func};

pub struct Layout<C: Component> {
    pub(crate) component: C,
    pub(crate) props: C::Props,
    pub(crate) children: Children<C::Target>,
}

impl<C: Component> Layout<C> {
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

    fn call_render(&self, eval: &mut EvalContext) -> Children<C::Target> {
        self.component.render(Render {
            eval,
            children: &self.children,
            props: &self.props,
        })
    }
}

pub trait LayoutBuilder: Component + Sized {
    fn with(self, props: Self::Props) -> Layout<Self> {
        Layout {
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
            component: self,
            props: Default::default(),
            children: None.into(),
        }
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
            children: None.into(),
        }
    }
}

impl<C> LayoutBuilder for C where C: Component {}

pub trait AnyLayout<T: Target> {
    fn props(&self) -> &dyn Any;
    fn mount(&self, ctx: &mut T::Mount, tree: Children<T>) -> Result<T::Result, T::Error>;
    fn render(&self, ev: &mut EvalContext) -> Children<T>;
}

impl<C: Component> AnyLayout<C::Target> for Layout<C> {
    fn props(&self) -> &dyn Any {
        &self.props as &dyn Any
    }

    fn mount(
        &self,
        ctx: &mut <C::Target as Target>::Mount,
        tree: Children<C::Target>,
    ) -> Result<<C::Target as Target>::Result, <C::Target as Target>::Error> {
        self.component.mount(ctx, tree)
    }

    fn render(&self, ev: &mut EvalContext) -> Children<C::Target> {
        self.call_render(ev)
    }
}
