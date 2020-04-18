use crate::{instance::Instance, mount::Mount, runtime::Runtime};
use std::rc::Rc;

pub trait Target: Clone + 'static {
    type Mount;
    type Result;
    type Error;
    type Runtime;

    fn mount_component(ctx: &mut Mount<Self>) -> Result<Self::Result, Self::Error>;

    fn mount(mount: Rc<Instance<Self>>) -> Result<Self::Result, Self::Error>;
    fn tick(scheduler: Rc<Runtime<Self>>);
}
