use std::{
    cell::{Ref, RefCell, RefMut},
    rc::{Rc, Weak},
};

use crate::instance::{Instance, InstanceDef};
use crate::{target::Target, Child};

pub struct Runtime<T: Target> {
    state: RefCell<RuntimeState<T>>,
}

pub struct RuntimeState<T: Target> {
    to_render: Vec<Weak<Instance<T>>>,
    to_update: Vec<Weak<Instance<T>>>,
    tick_scheduled: bool,
    tick_manually: bool,
}

impl<T: Target> Runtime<T> {
    pub fn new() -> Rc<Self> {
        let state = RuntimeState {
            to_render: vec![],
            to_update: vec![],
            tick_scheduled: false,
            tick_manually: false,
        };

        Rc::new(Runtime {
            state: RefCell::new(state),
        })
    }

    pub fn manual() -> Rc<Self> {
        let rt = Self::new();
        rt.state_mut().tick_manually = true;
        rt
    }

    pub(crate) fn state(&self) -> Ref<RuntimeState<T>> {
        self.state.borrow()
    }

    pub(crate) fn state_mut(&self) -> RefMut<RuntimeState<T>> {
        self.state.borrow_mut()
    }

    pub(crate) fn schedule_render(self: &Rc<Self>, inst: Weak<Instance<T>>) {
        self.state_mut().to_render.push(inst);
        self.schedule_tick()
    }

    pub(crate) fn schedule_update(self: &Rc<Self>, inst: Weak<Instance<T>>) {
        self.state_mut().to_update.push(inst);
        self.schedule_tick()
    }

    pub(crate) fn schedule_tick(self: &Rc<Self>) {
        if self.state().tick_scheduled {
            return;
        }

        self.state_mut().tick_scheduled = true;
        T::tick(self.clone())
    }

    pub fn render(
        self: &Rc<Self>,
        layout: Rc<dyn Child<T>>,
    ) -> Result<(T::Result, Rc<Instance<T>>), T::Error> {
        let instance = Instance::new(InstanceDef {
            runtime: self.clone(),
            level: 0,
            parent: None,
            layout,
        });
        let res = instance.perform_render()?;
        Ok((res, instance))
    }

    pub fn tick(&self) -> Result<(), T::Error> {
        let mut state = self.state_mut();
        state.tick_scheduled = false;

        let mut to_render = state
            .to_render
            .iter()
            .filter_map(|inst| inst.upgrade())
            .collect::<Vec<_>>();

        to_render.sort_by_key(|v| v.spec.level);
        for rt in to_render {
            rt.perform_render()?;
        }

        state.to_render.clear();

        let mut to_update = state
            .to_update
            .iter()
            .filter_map(|inst| inst.upgrade())
            .collect::<Vec<_>>();

        to_update.sort_by_key(|v| v.spec.level);
        for rt in to_update {
            rt.perform_update()?;
        }

        state.to_update.clear();

        Ok(())
    }
}
