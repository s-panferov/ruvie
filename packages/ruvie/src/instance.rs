use downcast_rs::{impl_downcast, Downcast};

use crate::{
	context::{AfterRender, Render},
	target::Target,
	Children,
};

pub trait Instance<T: Target>: Downcast {
	fn name(&self) -> &'static str;
	fn render(&mut self, eval: &Render<T>) -> Children<T>;
	fn should_render(&self) -> bool {
		true
	}

	fn debug(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "View")
	}

	fn mount(&mut self, ctx: &mut T::Mount) -> Result<(), T::Error> {
		T::mount_component(ctx)
	}

	fn after_render(&mut self, _ctx: &mut AfterRender<T>) {}
	fn before_unmount(&mut self) {}
}

impl_downcast!(Instance<T> where T: Target);
