use crate::{context::Mount, runtime::Runtime};
use std::rc::Rc;

pub trait Target: Clone + 'static {
    type Mount;
    type Result;
    type Error;
    type State;

    fn mount_component(ctx: &mut Self::Mount) -> Result<Self::Result, Self::Error>;
    fn mount(ctx: Mount<Self>) -> Result<(Self::Result, Mount<Self>), Self::Error>;
    fn tick(scheduler: Rc<Runtime<Self>>);
}
