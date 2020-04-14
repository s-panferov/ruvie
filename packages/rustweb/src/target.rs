use crate::{instance::Instance, scheduler::WeakScheduler, Children};

pub trait Target: Clone + 'static {
    type Mount;
    type Result;
    type Error;
    type Runtime;

    fn component(ctx: &mut Self::Mount, tree: Children<Self>) -> Result<Self::Result, Self::Error>;
    fn mount(mount: &mut Instance<Self>) -> Result<Self::Result, Self::Error>;
    fn tick(scheduler: WeakScheduler<Self>);
}
