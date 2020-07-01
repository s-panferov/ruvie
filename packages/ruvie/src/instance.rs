use downcast_rs::{impl_downcast, Downcast};

use crate::{
	context::{AfterRender, Mount, Render},
	error::RuvieError,
	Children,
};
use std::any::Any;

pub trait Instance: Downcast {
	fn name(&self) -> &'static str;
	fn render(&mut self, eval: &Render) -> Children;
	fn should_render(&self) -> bool {
		true
	}

	fn debug(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "View")
	}

	fn mount(&mut self, ctx: &mut Mount, target: &mut dyn Any) -> Result<(), RuvieError> {
		let rt = ctx.view.def.runtime.clone();
		let platform = &rt.platform;
		platform.mount_component(ctx, target)
	}

	fn after_render(&mut self, _ctx: &mut AfterRender) {}
	fn before_unmount(&mut self) {}
}

impl_downcast!(Instance);
