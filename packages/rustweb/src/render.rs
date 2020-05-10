use std::rc::Rc;

use crate::{
    instance::{Instance, InstanceDef},
    runtime::Runtime,
    target::Target,
    Child,
};

pub fn render<T: Target>(
    layout: Rc<dyn Child<T>>,
    parent: Option<Rc<Instance<T>>>,
) -> Result<(T::Result, Rc<Instance<T>>), T::Error> {
    let instance = Instance::new(InstanceDef {
        runtime: parent
            .as_ref()
            .map(|parent| parent.spec.runtime.clone())
            .unwrap_or_else(|| Runtime::new()),
        level: parent.as_ref().map(|p| p.spec.level).unwrap_or(0) + 1,
        parent: parent.map(|p| Rc::downgrade(&p)),
        layout,
    });
    let res = instance.perform_render()?;
    Ok((res, instance))
}
