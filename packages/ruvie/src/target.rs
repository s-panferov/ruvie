use crate::{context::Mount, error::RuvieError, runtime::Runtime, View};
use downcast_rs::Downcast;
use std::any::Any;

pub trait Target: Downcast {
	fn mount_component(&self, ctx: &mut Mount, target: &mut dyn Any) -> Result<(), RuvieError>;
	fn schedule_tick(&self, scheduler: &Runtime);
	fn mount(
		&self,
		view: &View,
		ctx: &mut Mount,
		arg: Option<Box<dyn Any>>,
	) -> Result<(), RuvieError>;
}

downcast_rs::impl_downcast!(Target);
