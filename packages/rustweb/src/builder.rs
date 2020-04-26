use crate::{
    component::{Component, ComponentExt},
    context::Render,
    props::{PropFor, Props},
    reference::BoundComponentRef,
    Children, Func, Layout,
};

use std::{hash::Hash, rc::Rc};

pub struct LayoutBuilder<C: Component> {
    component: C,
    props: Rc<C::Props>,
    children: Children<C::Target>,
    reference: Option<BoundComponentRef<C>>,
}

impl<C> LayoutBuilder<C>
where
    C: Component,
{
    pub fn new(component: C, props: Rc<C::Props>) -> Self {
        LayoutBuilder {
            component,
            props,
            children: None.into(),
            reference: None,
        }
    }

    pub fn with_ref(mut self, refr: BoundComponentRef<C>) -> Self {
        self.reference = Some(refr);
        self
    }

    pub fn prop<P: PropFor<C> + Hash, V: Into<P::Value>>(mut self, prop: P, value: V) -> Self
    where
        C: Component<Props = Props<C>>,
    {
        let props = Rc::get_mut(&mut self.props).unwrap();
        props.value_for(prop, value.into());
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
        F: Fn(&mut Render<(), C::Target>) -> CH + 'static,
        CH: Into<Children<C::Target>>,
    {
        let instance = Func::new(move |ctx| child(ctx).into());
        match self.children.as_mut() {
            Some(children) => children.push(Rc::new(Layout::from(instance.default()))),
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

    pub fn build(self) -> Layout<C> {
        Layout::from(self)
    }
}

impl<C> From<LayoutBuilder<C>> for Layout<C>
where
    C: Component,
{
    fn from(builder: LayoutBuilder<C>) -> Self {
        Layout {
            component: builder.component,
            children: builder.children,
            reference: builder.reference,
            props: builder.props,
        }
    }
}

impl<C> From<LayoutBuilder<C>> for Children<C::Target>
where
    C: Component,
{
    fn from(builder: LayoutBuilder<C>) -> Self {
        Layout::from(builder).into()
    }
}
