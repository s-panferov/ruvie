use std::cell::{Ref, RefCell, RefMut};
use std::{
    collections::{HashMap, HashSet},
    rc::{Rc, Weak},
};

use snowflake::ProcessUniqueId;

use observe::{
    local::{EvalContext, Local},
    Evaluation, Tracker, WeakTracker,
};

use crate::children::{Child, Children};
use crate::runtime::Runtime;
use crate::{after::AfterRender, mount::Reactions, target::Target};

pub struct InstanceSpec<T: Target> {
    pub runtime: Rc<Runtime<T>>,
    pub parent: Option<Weak<Instance<T>>>,
    pub layout: Child<T>,
    pub level: usize,
}

pub struct InstanceState<T: Target> {
    pub(crate) rendered_tree: Children<T>,
    pub(crate) children: Vec<Rc<Instance<T>>>,
    render_rx: Tracker,
    is_render_scheduled: bool,
    is_update_scheduled: bool,
    pub(crate) update_res: Result<(), T::Error>,
    pub(crate) all_update_reactions: Vec<Tracker<Local>>,
    pub(crate) invalidated_updates: HashSet<WeakTracker<Local>>,
    pub(crate) runtime: Option<T::Runtime>,
    pub(crate) references: HashMap<ProcessUniqueId, Weak<Instance<T>>>,
}

impl<T: Target> InstanceState<T> {
    pub fn new() -> Self {
        InstanceState {
            children: vec![],
            rendered_tree: None.into(),
            render_rx: Tracker::new("Render".to_owned()),
            all_update_reactions: vec![],
            invalidated_updates: HashSet::new(),
            update_res: Ok(()),
            is_render_scheduled: false,
            is_update_scheduled: false,
            runtime: None,
            references: HashMap::new(),
        }
    }
}

pub struct Instance<T: Target> {
    pub(crate) spec: InstanceSpec<T>,
    pub(crate) state: RefCell<InstanceState<T>>,
}

impl<T: Target> Instance<T> {
    pub fn new(spec: InstanceSpec<T>) -> Rc<Self> {
        let state = InstanceState::new();
        let instance = Rc::new(Instance {
            spec,
            state: RefCell::new(state),
        });

        {
            let self_ref = Rc::downgrade(&instance);
            let state = instance.state_mut();
            state
                .render_rx
                .set_computation(Box::new(RenderEngine { instance: self_ref }));
            state.render_rx.autorun();
        }

        instance
    }

    pub(crate) fn state(&self) -> Ref<InstanceState<T>> {
        self.state.borrow()
    }

    pub(crate) fn state_mut(&self) -> RefMut<InstanceState<T>> {
        self.state.borrow_mut()
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
        let res = T::mount(self.clone());
        self.perform_update()?;
        res
    }

    pub(crate) fn register_reference(
        self: &Rc<Self>,
        id: ProcessUniqueId,
        inst: Weak<Instance<T>>,
    ) {
        self.state_mut().references.insert(id, inst);
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

    pub(crate) fn after_render(self: &Rc<Self>) -> AfterRender<T> {
        let mut ctx = AfterRender {
            instance: self.clone(),
            reactions: Reactions::new(Rc::downgrade(&self)),
        };

        self.spec.layout.after_render(&mut ctx);
        ctx
    }

    pub(crate) fn before_unmount(&mut self) {
        self.spec.layout.before_unmount()
    }

    pub(crate) fn get(&self, refr: &dyn AsRef<ProcessUniqueId>) -> Option<Rc<Instance<T>>> {
        let state = self.state();
        let res = state.references.get(&refr.as_ref());
        res.and_then(|inst| inst.upgrade())
    }
}

struct RenderEngine<T>
where
    T: Target,
{
    instance: Weak<Instance<T>>,
}

impl<T> Evaluation<Local> for RenderEngine<T>
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
