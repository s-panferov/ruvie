use std::{
	cell::{Ref, RefCell, RefMut},
	sync::Arc,
};

use crate::view::{View, ViewDef, WeakView};
use crate::{target::Target, Element};

pub struct Runtime<T: Target> {
	body: Arc<RuntimeBody<T>>,
}

impl<T> Clone for Runtime<T>
where
	T: Target,
{
	fn clone(&self) -> Self {
		Runtime {
			body: self.body.clone(),
		}
	}
}

pub struct RuntimeBody<T: Target> {
	state: RefCell<RuntimeMut<T>>,
}

pub struct RuntimeMut<T: Target> {
	to_render: Vec<WeakView<T>>,
	to_update: Vec<WeakView<T>>,
	tick_scheduled: bool,
	tick_manually: bool,
}

impl<T: Target> Runtime<T> {
	pub fn new() -> Self {
		let state = RuntimeMut {
			to_render: vec![],
			to_update: vec![],
			tick_scheduled: false,
			tick_manually: false,
		};

		Runtime {
			body: Arc::new(RuntimeBody {
				state: RefCell::new(state),
			}),
		}
	}

	pub fn manual() -> Self {
		let rt = Self::new();
		rt.state_mut().tick_manually = true;
		rt
	}

	pub(crate) fn state(&self) -> Ref<RuntimeMut<T>> {
		self.body.state.borrow()
	}

	pub(crate) fn state_mut(&self) -> RefMut<RuntimeMut<T>> {
		self.body.state.borrow_mut()
	}

	pub(crate) fn schedule_render(&self, inst: WeakView<T>) {
		self.state_mut().to_render.push(inst);
		self.schedule_tick()
	}

	pub(crate) fn schedule_update(&self, inst: WeakView<T>) {
		self.state_mut().to_update.push(inst);
		self.schedule_tick()
	}

	pub(crate) fn schedule_tick(&self) {
		if self.state().tick_scheduled || self.state().tick_manually {
			return;
		}

		self.state_mut().tick_scheduled = true;
		T::schedule_tick(&self)
	}

	pub fn render(&self, element: impl Into<Element<T>>, arg: T::Arg) -> Result<View<T>, T::Error> {
		let instance = View::new(ViewDef {
			runtime: self.clone(),
			level: 0,
			parent: None,
			element: element.into(),
		});
		instance.perform_render(Some(arg))?;
		Ok(instance)
	}

	pub fn tick(&self) -> Result<(), T::Error> {
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
