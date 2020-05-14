use crate::{
    component::{Component, ComponentExt},
    context::Render,
    props::{PropFor, Props},
    reference::{BoundRef, CompatibleReference},
    target::Target,
    Children, Func, Layout,
};

use std::{hash::Hash, rc::Rc};

pub struct LayoutBuilder<C: Component<T>, T>
where
    T: Target,
{
    component: C,
    props: Rc<C::Props>,
    children: Children<T>,
    reference: Option<BoundRef<T>>,
}

impl<C, T> LayoutBuilder<C, T>
where
    C: Component<T>,
    T: Target,
{
    pub fn new(component: C, props: Rc<C::Props>) -> Self {
        LayoutBuilder {
            component,
            props,
            children: None.into(),
            reference: None,
        }
    }

    pub fn with_ref(mut self, refr: impl CompatibleReference<C, T>) -> Self {
        self.reference = Some(refr.to_bound_ref());
        self
    }

    pub fn prop<P: PropFor<C> + Hash, V: Into<P::Value>>(mut self, prop: P, value: V) -> Self
    where
        C: Component<T, Props = Props<C>>,
    {
        let props = Rc::get_mut(&mut self.props).unwrap();
        props.value_for(prop, value.into());
        self
    }

    pub fn child<F, CH>(mut self, child: F) -> Self
    where
        F: Into<Layout<CH, T>>,
        CH: Component<T> + 'static,
    {
        match self.children.as_mut() {
            Some(children) => children.push(Rc::new(child.into())),
            None => self.children = child.into().into(),
        }

        self
    }

    pub fn scope<F, CH>(mut self, child: F) -> Self
    where
        F: Fn(&mut Render<(), T>) -> CH + 'static,
        CH: Into<Children<T>>,
    {
        let instance = Func::new(move |ctx| child(ctx).into());
        match self.children.as_mut() {
            Some(children) => children.push(Rc::new(Layout::from(instance.default()))),
            None => self.children = instance.default().into(),
        }

        self
    }

    pub fn children(mut self, mut children: Children<T>) -> Self {
        if children.is_none() {
            return self;
        }

        match self.children.as_mut() {
            Some(current) => current.append(children.as_mut().unwrap()),
            None => self.children = children,
        }

        self
    }

    pub fn build(self) -> Layout<C, T> {
        Layout::from(self)
    }
}

impl<C, T> From<LayoutBuilder<C, T>> for Layout<C, T>
where
    C: Component<T>,
    T: Target,
{
    fn from(builder: LayoutBuilder<C, T>) -> Self {
        Layout {
            component: builder.component,
            children: builder.children,
            reference: builder.reference,
            props: builder.props,
        }
    }
}

impl<C, T> From<LayoutBuilder<C, T>> for Children<T>
where
    C: Component<T>,
    T: Target,
{
    fn from(builder: LayoutBuilder<C, T>) -> Self {
        Layout::from(builder).into()
    }
}
