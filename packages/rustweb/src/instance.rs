use std::sync::{Arc, Weak};
use std::{
    collections::HashSet,
    sync::{RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use observe::EvalContext;
use observe::{Tracker, WeakTracker};

use crate::component::Target;
use crate::layout::Child;
use crate::layout::Children;
use crate::scheduler::Scheduler;

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
            rx.get_mut().update();
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
                rx.get_mut().update();
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

pub struct Instance<T: Target> {
    pub(crate) opts: InstanceSpec<T>,
    pub(crate) tree: Children<T>,
    pub(crate) children: Vec<InstanceRef<T>>,
    reference: Option<WeakInstance<T>>,
    render_rx: Tracker,
    render_scheduled: bool,
    update_scheduled: bool,
    pub(crate) update_res: Result<(), T::Error>,
    pub(crate) update_rx: Vec<Tracker>,
    pub(crate) updated_rx: HashSet<WeakTracker>,
    pub(crate) runtime: Option<T::Runtime>,
}

pub struct InstanceSpec<T: Target> {
    pub scheduler: Scheduler<T>,
    pub parent: Option<WeakInstance<T>>,
    pub layout: Child<T>,
    pub level: usize,
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
        let mut render_rx = self.render_rx.get_mut();

        render_rx.set_is_observer();
        render_rx.set_computation({
            let instance = self.weak_ref().clone();
            move |eval: &mut EvalContext| {
                if let Some(i) = instance.upgrade() {
                    i.get_mut().render(eval);
                } else {
                    unreachable!()
                }
                0
            }
        });

        render_rx.on_reaction({
            let rt = self.weak_ref().clone();
            move || {
                if let Some(instance) = rt.upgrade() {
                    // FIXME triggers several times, need to optimize in observe
                    instance.schedule_render()
                } else {
                    unreachable!()
                }
            }
        });
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
