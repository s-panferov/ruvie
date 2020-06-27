use crate::{context::Mount, error::RuvieError, target::Target, Runtime, View};
use std::any::Any;

#[derive(Clone)]
pub struct StaticHtml {}

impl Target for StaticHtml {
	fn mount(
		&self,
		_view: &View,
		ctx: &mut Mount,
		_arg: Option<Box<dyn Any>>,
	) -> Result<(), RuvieError> {
		Ok(())
	}

	fn mount_component(&self, _ctx: &mut Mount, _target: &mut dyn Any) -> Result<(), RuvieError> {
		Ok(())
	}

	fn schedule_tick(&self, _scheduler: &Runtime) {}
}
