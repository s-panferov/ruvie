use std::cell::{Ref, RefCell, RefMut};
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    rc::{Rc, Weak},
};

use observe::{
    local::{EvalContext, Local},
    Evaluation, Tracker, WeakTracker,
};

use crate::children::Children;
use crate::runtime::Runtime;
use crate::{
    context::{AfterRender, Mount, Update},
    reference::BoundRef,
    target::Target,
    Child,
};

pub struct InstanceDef<T: Target> {
    pub runtime: Rc<Runtime<T>>,
    pub parent: Option<Weak<Instance<T>>>,
    pub layout: Rc<dyn Child<T>>,
    pub level: usize,
}

pub struct Instance<T: Target> {
    pub(crate) spec: InstanceDef<T>,
    state: RefCell<InstanceState<T>>,
}

pub struct InstanceState<T: Target> {
    rendered_tree: Children<T>,
    pub children: Vec<Rc<Instance<T>>>,
    render_rx: Tracker,
    is_render_scheduled: bool,
    is_update_scheduled: bool,
    update_res: Result<(), T::Error>,
    all_update_reactions: Vec<Tracker<Local>>,
    invalidated_updates: HashSet<WeakTracker<Local>>,
    references: HashMap<u64, Weak<Instance<T>>>,
    pub(crate) target: Option<T::State>,
}

impl<T: Target> Instance<T> {
    pub fn new(spec: InstanceDef<T>) -> Rc<Self> {
        let state = InstanceState {
            children: vec![],
            rendered_tree: None.into(),
            render_rx: Tracker::new("Render".to_owned()),
            all_update_reactions: vec![],
            invalidated_updates: HashSet::new(),
            update_res: Ok(()),
            is_render_scheduled: false,
            is_update_scheduled: false,
            target: None,
            references: HashMap::new(),
        };

        let instance = Rc::new(Instance {
            spec,
            state: RefCell::new(state),
        });

        {
            let state = instance.state_mut();
            state.render_rx.set_computation(Box::new(RenderReaction {
                instance: Rc::downgrade(&instance),
            }));
            state.render_rx.autorun();
        }

        instance
    }

    pub(crate) fn add_child(&self, child: Rc<Instance<T>>) {
        self.state_mut().children.push(child)
    }

    pub(crate) fn state(&self) -> Ref<InstanceState<T>> {
        self.state.borrow()
    }

    pub(crate) fn state_mut(&self) -> RefMut<InstanceState<T>> {
        self.state.borrow_mut()
    }

    pub fn render_child(
        self: &Rc<Self>,
        layout: Rc<dyn Child<T>>,
    ) -> Result<(T::Result, Rc<Instance<T>>), T::Error> {
        let instance = Instance::new(InstanceDef {
            runtime: self.spec.runtime.clone(),
            level: self.spec.level + 1,
            parent: Some(Rc::downgrade(&self)),
            layout,
        });
        let res = instance.perform_render()?;
        Ok((res, instance))
    }

    pub(crate) fn render(self: &Rc<Instance<T>>, eval: &mut EvalContext) {
        self.state_mut().rendered_tree = self.spec.layout.render(self.clone(), eval);
    }

    pub(crate) fn perform_render(self: &Rc<Self>) -> Result<T::Result, T::Error> {
        {
            let rx = self.state().render_rx.clone();
            rx.update();
        }

        self.state_mut().is_render_scheduled = false;
        let res = self.mount();
        self.perform_update()?;
        res
    }

    pub(crate) fn reaction(
        self: &Rc<Self>,
        handler: Box<dyn for<'a> Fn(&'a Self, &'a mut Update<'a, T>) -> Result<(), T::Error>>,
    ) {
        let rx = Tracker::new("Update Reaction".to_owned());
        rx.set_computation(Box::new(UpdateReaction {
            handler,
            instance: Rc::downgrade(&self),
            rx: rx.weak(),
        }));

        rx.autorun();

        self.state_mut().invalidated_updates.insert(rx.weak());
        self.state_mut().all_update_reactions.push(rx);
    }

    pub(crate) fn mount(self: &Rc<Self>) -> Result<T::Result, T::Error> {
        let tree = self.state_mut().rendered_tree.take();
        let ctx = Mount {
            children: vec![],
            instance: self.clone(),
            tree,
        };

        let (res, ctx) = T::mount(ctx)?;
        let Mount { children, .. } = ctx;

        self.state_mut().children = children;

        if let Some(refr) = self.spec.layout.get_ref() {
            refr.apply(Rc::downgrade(&self))
        }

        self.after_render();

        Ok(res)
    }

    pub fn reference_any<K: Hash>(self: &Rc<Self>, reference: &K) -> BoundRef<T> {
        BoundRef {
            parent: Rc::downgrade(&self),
            id: fxhash::hash64(&reference),
        }
    }

    pub fn register_reference<K: Hash>(self: &Rc<Self>, id: &K, inst: Weak<Instance<T>>) {
        self.state_mut().references.insert(fxhash::hash64(id), inst);
    }

    pub(crate) fn perform_update(&self) -> Result<(), T::Error> {
        // clone to release self-reference
        let updated = self.state().invalidated_updates.clone();

        for rx in updated {
            let rx = rx.upgrade();
            if let Some(rx) = rx {
                rx.update();
                let res = std::mem::replace(&mut self.state_mut().update_res, Ok(()));
                if res.is_err() {
                    return res;
                }
            }
        }

        self.state_mut().is_update_scheduled = false;
        Ok(())
    }

    pub(crate) fn schedule_render(self: &Rc<Self>) {
        if self.state().is_render_scheduled {
            return;
        }

        self.state_mut().is_render_scheduled = true;
        self.spec.runtime.schedule_render(Rc::downgrade(&self));
    }

    pub(crate) fn schedule_update(self: &Rc<Self>) {
        if self.state().is_update_scheduled {
            return;
        }

        self.state_mut().is_update_scheduled = true;
        self.spec.runtime.schedule_update(Rc::downgrade(&self));
    }

    pub(crate) fn after_render(self: &Rc<Self>) {
        let mut ctx = AfterRender {
            instance: self.clone(),
        };

        self.spec.layout.after_render(&mut ctx);
    }

    pub(crate) fn before_unmount(&mut self) {
        self.spec.layout.before_unmount()
    }

    pub(crate) fn get<K: Hash>(&self, refr: &K) -> Option<Rc<Instance<T>>> {
        let state = self.state();
        let res = state.references.get(&fxhash::hash64(refr));
        res.and_then(|inst| inst.upgrade())
    }
}

struct RenderReaction<T>
where
    T: Target,
{
    instance: Weak<Instance<T>>,
}

impl<T> Evaluation<Local> for RenderReaction<T>
where
    T: Target,
{
    fn evaluate(&mut self, ctx: &mut EvalContext) -> u64 {
        if let Some(i) = self.instance.upgrade() {
            i.render(ctx);
        } else {
            unreachable!()
        }
        0
    }

    fn on_reaction(&mut self) {
        if let Some(instance) = self.instance.upgrade() {
            // FIXME triggers several times, need to optimize in observe
            instance.schedule_render()
        } else {
            unreachable!()
        }
    }

    fn is_scheduled(&self) -> bool {
        true
    }
}

pub struct UpdateReaction<T>
where
    T: Target,
{
    pub instance: Weak<Instance<T>>,
    pub rx: WeakTracker<Local>,
    pub handler:
        Box<dyn for<'a> Fn(&'a Instance<T>, &'a mut Update<'a, T>) -> Result<(), T::Error>>,
}

impl<T> Evaluation<Local> for UpdateReaction<T>
where
    T: Target,
{
    fn is_scheduled(&self) -> bool {
        true
    }

    fn evaluate(&mut self, eval: &mut EvalContext) -> u64 {
        if let Some(instance) = self.instance.upgrade() {
            let inst = instance.clone();
            let mut ctx = Update { eval, instance };
            let res = (self.handler)(&inst, &mut ctx);
            inst.state_mut().update_res = res;
        } else {
            unreachable!()
        }
        0
    }

    fn on_reaction(&mut self) {
        if let Some(instance) = self.instance.upgrade() {
            // FIXME move to instance
            instance
                .state_mut()
                .invalidated_updates
                .insert(self.rx.clone());
            // FIXME triggers several times, need to optimize in observe
            instance.schedule_update()
        } else {
            unreachable!()
        }
    }
}
