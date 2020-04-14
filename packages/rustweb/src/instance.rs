use std::sync::{Arc, Weak};
use std::{
    collections::HashSet,
    sync::{RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use observe::{
    local::{EvalContext, Local},
    Evaluation, Tracker, WeakTracker,
};

use crate::children::{Child, Children};
use crate::scheduler::Scheduler;
use crate::target::Target;

#[derive(Clone)]
pub struct InstanceRef<T: Target> {
    level: usize,
    body: Arc<RwLock<Instance<T>>>,
}

impl<T: Target> InstanceRef<T> {
    pub fn new(opts: InstanceSpec<T>) -> Self {
        let level = opts.level;
        let body = Instance::new(opts);
        let runtime = InstanceRef {
            level,
            body: Arc::new(RwLock::new(body)),
        };

        let _re = runtime.weak();

        {
            let mut rt = runtime.get_mut();
            rt.reference = Some(_re);
            rt.init();
        }

        runtime
    }

    pub fn weak(&self) -> WeakInstance<T> {
        return WeakInstance {
            level: self.level,
            body: Arc::downgrade(&self.body),
        };
    }

    pub fn get(&self) -> RwLockReadGuard<Instance<T>> {
        self.body.read().unwrap()
    }

    pub fn get_mut(&self) -> RwLockWriteGuard<Instance<T>> {
        self.body.write().unwrap()
    }

    pub fn perform_render(&self) -> Result<T::Result, T::Error> {
        {
            let rx = self.get().render_rx.clone();
            rx.update();
        }

        self.get_mut().render_scheduled = false;
        let res = T::mount(&mut self.get_mut());
        self.perform_update()?;
        res
    }

    pub fn perform_update(&self) -> Result<(), T::Error> {
        // clone to release self-reference
        let updated = self.get().updated_rx.clone();

        for rx in updated {
            let rx = rx.upgrade();
            if let Some(rx) = rx {
                rx.update();
                let res = std::mem::replace(&mut self.get_mut().update_res, Ok(()));
                if res.is_err() {
                    return res;
                }
            }
        }

        self.get_mut().update_scheduled = false;
        Ok(())
    }

    pub fn schedule_render(&self) {
        if self.get().render_scheduled {
            return;
        }

        self.get_mut().render_scheduled = true;

        // Clone here because scheduler may be syncronous
        let scheduler = self.get().opts.scheduler.clone();
        scheduler.schedule_render(self.weak());
    }

    pub fn schedule_update(&self) {
        if self.get().update_scheduled {
            return;
        }

        self.get_mut().update_scheduled = true;

        // Clone here because scheduler may be syncronous
        let scheduler = self.get().opts.scheduler.clone();
        scheduler.schedule_update(self.weak());
    }
}

pub struct InstanceSpec<T: Target> {
    pub scheduler: Scheduler<T>,
    pub parent: Option<WeakInstance<T>>,
    pub layout: Child<T>,
    pub level: usize,
}

struct RenderEngine<T>
where
    T: Target,
{
    instance: WeakInstance<T>,
}

impl<T> Evaluation<Local> for RenderEngine<T>
where
    T: Target,
{
    fn evaluate(&mut self, ctx: &mut EvalContext) -> u64 {
        if let Some(i) = self.instance.upgrade() {
            i.get_mut().render(ctx);
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

pub struct Instance<T: Target> {
    pub(crate) opts: InstanceSpec<T>,
    pub(crate) tree: Children<T>,
    pub(crate) children: Vec<InstanceRef<T>>,
    reference: Option<WeakInstance<T>>,
    render_rx: Tracker,
    render_scheduled: bool,
    update_scheduled: bool,
    pub(crate) update_res: Result<(), T::Error>,
    pub(crate) update_rx: Vec<Tracker<Local>>,
    pub(crate) updated_rx: HashSet<WeakTracker<Local>>,
    pub(crate) runtime: Option<T::Runtime>,
}

impl<T: Target> Instance<T> {
    pub fn new(opts: InstanceSpec<T>) -> Self {
        Instance {
            opts,
            children: vec![],
            tree: None.into(),
            reference: None,
            render_rx: Tracker::new("Render".to_owned()),
            update_rx: vec![],
            updated_rx: HashSet::new(),
            update_res: Ok(()),
            render_scheduled: false,
            update_scheduled: false,
            runtime: None,
        }
    }

    pub fn init(&mut self) {
        self.render_rx.set_computation(Box::new(RenderEngine {
            instance: self.weak_ref().clone(),
        }));
        self.render_rx.autorun();
    }

    pub fn weak_ref(&self) -> &WeakInstance<T> {
        self.reference.as_ref().unwrap()
    }

    pub fn strong_ref(&self) -> InstanceRef<T> {
        self.weak_ref().upgrade().unwrap()
    }

    pub fn render(&mut self, eval: &mut EvalContext) {
        self.tree = self.opts.layout.render(eval);
    }
}

#[derive(Clone)]
pub struct WeakInstance<T: Target> {
    pub level: usize,
    body: Weak<RwLock<Instance<T>>>,
}

impl<T: Target> WeakInstance<T> {
    pub fn upgrade(&self) -> Option<InstanceRef<T>> {
        let body = self.body.upgrade();
        body.map(|body| InstanceRef {
            level: self.level,
            body,
        })
    }
}
