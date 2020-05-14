use crate::children::Children;
use crate::{
    builder::LayoutBuilder,
    context::{AfterRender, Handler, Render, Update},
    props::PropFor,
    reference::BoundComponentRef,
    target::Target,
    ComponentRef, Event, Instance, Props,
};

use std::{any::Any, hash::Hash, rc::Rc};

/// Basic system trait all components should implement
pub trait Component<T: Target>: 'static + Sized {
    type Props: Any;

    /// Defines component name. Useful for debugging.
    fn name(&self) -> &'static str {
        return "Component";
    }

    fn should_render(&self, _props: &Self::Props) -> bool {
        true
    }

    /// Main function that defines component layout
    fn render(&self, _ctx: &mut Render<Self::Props, T>) -> Children<T> {
        _ctx.children.clone()
    }

    fn after_render(&self, _ctx: &mut AfterRender<T>) {}

    fn before_unmount(&self) {}

    fn mount(&self, ctx: &mut T::Mount) -> Result<T::Result, T::Error> {
        T::mount_component(ctx)
    }
}

pub trait ComponentExt<T>: Component<T>
where
    T: Target,
{
    /// Get props from various available contexts
    fn props(ctx: &Rc<Instance<T>>) -> Rc<Self::Props> {
        ctx.spec
            .layout
            .props()
            .downcast::<Self::Props>()
            .expect("Type")
    }

    /// Wrap a reference
    fn reference<C: Component<T>>(
        instance: &Rc<Instance<T>>,
        reference: &ComponentRef<C, T>,
    ) -> BoundComponentRef<C, T> {
        reference.bind(&instance)
    }

    /// Wrap a handler function to create an Event object that can
    /// sent to another component as an event handler
    fn handler<F, E>(instance: &Rc<Instance<T>>, handler: F) -> Event<E>
    where
        F: Fn(&Self, Handler<E, T>) + 'static,
        E: 'static,
    {
        let instance = Rc::downgrade(&instance);
        let handler = Rc::new(handler);
        Event::new(move |event| {
            if let Some(instance) = instance.upgrade() {
                let instance_1 = instance.clone();
                let handler = handler.clone();
                let component = instance
                    .spec
                    .layout
                    .component()
                    .downcast_ref::<Self>()
                    .expect("Component");

                handler(
                    &component,
                    Handler {
                        event,
                        instance: instance_1,
                    },
                );
            }
        })
    }

    /// Wrap a reaction callback to be run in the context of the component
    fn reaction<F, R>(
        handler: F,
    ) -> Box<dyn for<'a> Fn(&'a Instance<T>, &'a mut Update<'a, T>) -> R>
    where
        F: for<'a> Fn(&'a Self, &mut Update<'a, T>) -> R + 'static,
    {
        Box::new(move |instance, ctx| {
            let component = instance
                .spec
                .layout
                .component()
                .downcast_ref::<Self>()
                .expect("Type");
            (handler)(component, ctx)
        })
    }

    fn with_props(self, props: Rc<Self::Props>) -> LayoutBuilder<Self, T> {
        LayoutBuilder::new(self, props)
    }

    fn prop<P: PropFor<Self> + Hash, V: Into<P::Value>>(
        self,
        prop: P,
        value: V,
    ) -> LayoutBuilder<Self, T>
    where
        Self: Component<T, Props = Props<Self>>,
    {
        let mut props = Props::new();
        props.value_for(prop, value.into());
        LayoutBuilder::new(self, Rc::new(props))
    }

    fn default(self) -> LayoutBuilder<Self, T>
    where
        Self::Props: Default,
    {
        LayoutBuilder::new(self, Default::default())
    }
}

impl<C, T> ComponentExt<T> for C
where
    C: Component<T>,
    T: Target,
{
}
