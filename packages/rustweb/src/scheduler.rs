use std::sync::{Arc, Weak};
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::component::Target;
use crate::instance::WeakInstance;

#[derive(Clone)]
pub struct Scheduler<T: Target> {
    body: Arc<RwLock<SchedulerBody<T>>>,
}

impl<T: Target> Scheduler<T> {
    pub fn new() -> Self {
        let body = SchedulerBody::new();
        let scheduler = Scheduler {
            body: Arc::new(RwLock::new(body)),
        };

        let _re = scheduler.weak();

        {
            let mut rt = scheduler.get_mut();
            rt.reference = Some(_re);
        }

        scheduler
    }

    pub fn weak(&self) -> WeakScheduler<T> {
        return WeakScheduler {
            body: Arc::downgrade(&self.body),
        };
    }

    pub fn get(&self) -> RwLockReadGuard<SchedulerBody<T>> {
        self.body.read().unwrap()
    }

    pub fn get_mut(&self) -> RwLockWriteGuard<SchedulerBody<T>> {
        self.body.write().unwrap()
    }

    pub fn schedule_render(&self, inst: WeakInstance<T>) {
        self.get_mut().to_render.push(inst);
        self.schedule_tick()
    }

    pub fn schedule_update(&self, inst: WeakInstance<T>) {
        self.get_mut().to_update.push(inst);
        self.schedule_tick()
    }

    pub fn schedule_tick(&self) {
        if self.get().tick_scheduled {
            return;
        }

        self.get_mut().tick_scheduled = true;
        T::tick(self.weak())
    }
}

pub struct SchedulerBody<T: Target> {
    to_render: Vec<WeakInstance<T>>,
    to_update: Vec<WeakInstance<T>>,
    tick_scheduled: bool,
    reference: Option<WeakScheduler<T>>,
}

impl<T: Target> SchedulerBody<T> {
    pub fn new() -> SchedulerBody<T> {
        SchedulerBody {
            to_render: vec![],
            to_update: vec![],
            tick_scheduled: false,
            reference: None,
        }
    }

    pub fn weak_ref(&self) -> &WeakScheduler<T> {
        self.reference.as_ref().unwrap()
    }

    pub fn strong_ref(&self) -> Scheduler<T> {
        self.weak_ref().upgrade().unwrap()
    }

    pub fn tick(&mut self) -> Result<(), T::Error> {
        self.tick_scheduled = false;

        self.to_render.sort_by_key(|v| v.level);
        for rt in self.to_render.iter() {
            match rt.upgrade() {
                Some(rt) => {
                    rt.perform_render()?;
                }
                None => {
                    // Node has gone
                }
            }
        }

        self.to_render.clear();

        self.to_update.sort_by_key(|v| v.level);
        for rt in self.to_update.iter().rev() {
            match rt.upgrade() {
                Some(rt) => {
                    rt.perform_update()?;
                }
                None => {
                    // Node has gone
                }
            }
        }

        self.to_update.clear();

        Ok(())
    }
}

#[derive(Clone)]
pub struct WeakScheduler<T: Target> {
    body: Weak<RwLock<SchedulerBody<T>>>,
}

impl<T: Target> WeakScheduler<T> {
    pub fn upgrade(&self) -> Option<Scheduler<T>> {
        let body = self.body.upgrade();
        body.map(|body| Scheduler { body })
    }
}
