use std::{
	any::Any,
	cell::{Ref, RefCell, RefMut},
	ops::Deref,
	sync::Arc,
};

use crate::view::{View, ViewDef, WeakView};
use crate::{error::RuvieError, target::Target, Element};

pub struct Runtime {
	body: Arc<RuntimeShared>,
}

pub struct RuntimeMut {
	to_render: Vec<WeakView>,
	to_update: Vec<WeakView>,
	tick_scheduled: bool,
	tick_manually: bool,
}

impl Clone for Runtime {
	fn clone(&self) -> Self {
		Runtime {
			body: self.body.clone(),
		}
	}
}

impl Deref for Runtime {
	type Target = RuntimeShared;
	fn deref(&self) -> &Self::Target {
		&self.body
	}
}

pub struct RuntimeShared {
	pub platform: Arc<dyn Target>,
	state: RefCell<RuntimeMut>,
}

impl Runtime {
	pub fn new<T: Target>(platform: T) -> Self {
		let state = RuntimeMut {
			to_render: vec![],
			to_update: vec![],
			tick_scheduled: false,
			tick_manually: false,
		};

		Runtime {
			body: Arc::new(RuntimeShared {
				platform: Arc::new(platform),
				state: RefCell::new(state),
			}),
		}
	}

	pub fn manual<T: Target>(target: T) -> Self {
		let rt = Self::new(target);
		rt.state_mut().tick_manually = true;
		rt
	}

	pub(crate) fn state(&self) -> Ref<RuntimeMut> {
		self.body.state.borrow()
	}

	pub(crate) fn state_mut(&self) -> RefMut<RuntimeMut> {
		self.body.state.borrow_mut()
	}

	pub(crate) fn schedule_render(&self, inst: WeakView) {
		self.state_mut().to_render.push(inst);
		self.schedule_tick()
	}

	pub(crate) fn schedule_update(&self, inst: WeakView) {
		self.state_mut().to_update.push(inst);
		self.schedule_tick()
	}

	pub(crate) fn schedule_tick(&self) {
		if self.state().tick_scheduled || self.state().tick_manually {
			return;
		}

		self.state_mut().tick_scheduled = true;
		self.body.platform.schedule_tick(&self);
	}

	pub fn render(
		&self,
		element: impl Into<Element>,
		arg: Box<dyn Any>,
	) -> Result<View, RuvieError> {
		let instance = View::new(ViewDef {
			runtime: self.clone(),
			level: 0,
			parent: None,
			element: element.into(),
		});
		instance.perform_render(Some(arg))?;
		Ok(instance)
	}

	pub fn tick(&self) -> Result<(), RuvieError> {
		let mut state = self.state_mut();
		// FIXME think how to deal with continous updates
		let to_render = std::mem::replace(&mut state.to_render, vec![]);
		let to_update = std::mem::replace(&mut state.to_update, vec![]);
		state.tick_scheduled = false;
		std::mem::drop(state);

		let mut to_render = to_render
			.iter()
			.filter_map(|inst| inst.upgrade())
			.collect::<Vec<_>>();

		to_render.sort_by_key(|instance| instance.level());
		for rt in to_render {
			rt.perform_render(None)?;
		}

		let mut to_update = to_update
			.iter()
			.filter_map(|inst| inst.upgrade())
			.collect::<Vec<_>>();

		to_update.sort_by_key(|instance| instance.level());
		for rt in to_update {
			rt.perform_update()?;
		}

		Ok(())
	}
}
