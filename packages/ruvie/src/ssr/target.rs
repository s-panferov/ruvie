use crate::{context::Mount, error::RuvieError, target::Target, Runtime, View};
use std::{any::Any, rc::Rc};

use super::stylesheet::StaticStyleRuntime;
use markup5ever_rcdom::Node;

pub struct Static {
	pub styles: StaticStyleRuntime,
}

impl Static {
	pub fn new() -> Self {
		Static {
			styles: StaticStyleRuntime::new(),
		}
	}
}

impl Target for Static {
	fn mount(
		&self,
		view: &View,
		ctx: &mut Mount,
		_arg: Option<Box<dyn Any>>,
	) -> Result<(), RuvieError> {
		let mut html = StaticContext { fragment: vec![] };

		view.with_instance(|component| component.mount(ctx, &mut html))?;

		let StaticContext { mut fragment, .. } = html;

		view.with_state(|state| {
			if state.is_none() {
				super::fragment::wrap_fragment(&mut fragment, view.name());
				*state = Some(Box::new(StaticElementState { fragment }))
			} else {
				unreachable!("Updates should not happen in static mode")
			}

			Ok::<(), RuvieError>(())
		})?;

		Ok(())
	}

	fn mount_component(&self, ctx: &mut Mount, target: &mut dyn Any) -> Result<(), RuvieError> {
		if target.is::<StaticContext>() {
			let target = target.downcast_mut::<StaticContext>().unwrap();
			super::utils::mount_children(ctx, target, None)
		} else {
			Ok(()) // FIXME should be an error
		}
	}

	fn schedule_tick(&self, _scheduler: &Runtime) {
		unreachable!("Ticks are not supported in static target")
	}
}

pub struct StaticContext {
	pub(crate) fragment: Vec<Rc<Node>>,
}

pub struct StaticElementState {
	pub(crate) fragment: Vec<Rc<Node>>,
}
