use std::{
	any::Any,
	collections::{HashMap, HashSet},
	hash::Hash,
	mem::ManuallyDrop,
	ops::Deref,
	sync::{Arc, Weak},
};

use parking_lot::{
	MappedRwLockReadGuard, MappedRwLockWriteGuard, RwLock, RwLockReadGuard, RwLockWriteGuard,
};

use observe::{EvalContext, Evaluation, Tracker, WeakTracker};

use crate::children::Children;
use crate::runtime::Runtime;
use crate::{
	context::{AfterRender, Mount, Render, Update},
	instance::Instance,
	reference::Reference,
};

use crate::{element::Element, error::RuvieError};

pub struct ViewDef {
	pub runtime: Runtime,
	pub parent: Option<WeakView>,
	pub element: Element,
	pub level: usize,
}

pub struct View {
	inner: Arc<ViewShared>,
}

impl Clone for View {
	fn clone(&self) -> Self {
		View {
			inner: self.inner.clone(),
		}
	}
}

impl Deref for View {
	type Target = ViewShared;
	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

pub struct ViewShared {
	pub(crate) def: ViewDef,
	render_rx: Tracker,
	pub component: RwLock<Option<Box<dyn Instance>>>,
	state: RwLock<ViewMut>,
}

pub struct ViewMut {
	rendered_tree: Children,
	pub children: ManuallyDrop<Vec<View>>,
	is_render_scheduled: bool,
	is_update_scheduled: bool,
	update_res: Result<(), RuvieError>,
	all_reactions: Vec<Tracker>,
	stale_reactions: HashSet<WeakTracker>,
	references: HashMap<u64, WeakView>,
	pub(crate) target: ManuallyDrop<Option<Box<dyn Any>>>,
}

pub type ReactionHandler = Box<dyn for<'a> Fn(&'a mut Update<'a>) -> Result<(), RuvieError>>;

impl Drop for ViewMut {
	fn drop(&mut self) {
		unsafe {
			let _ = ManuallyDrop::drop(&mut self.target);
			let _ = ManuallyDrop::drop(&mut self.children);
		}
	}
}

impl View {
	pub fn new(spec: ViewDef) -> Self {
		let state = ViewMut {
			children: ManuallyDrop::new(vec![]),
			rendered_tree: None.into(),
			all_reactions: vec![],
			stale_reactions: HashSet::new(),
			update_res: Ok(()),
			is_render_scheduled: false,
			is_update_scheduled: false,
			references: HashMap::new(),
			target: ManuallyDrop::new(None),
		};

		let view = View {
			inner: Arc::new(ViewShared {
				def: spec,
				component: RwLock::new(None),
				render_rx: Tracker::new(),
				state: RwLock::new(state),
			}),
		};

		*view.inner.component.write() =
			Some(view.inner.def.element.instance(View::downgrade(&view)));

		Tracker::set_eval(
			&view.inner.render_rx,
			Arc::new(RenderReaction {
				instance: View::downgrade(&view),
			}),
		);

		view.inner.render_rx.autorun();
		view.inner.render_rx.update();

		view
	}

	pub fn component(&self) -> MappedRwLockReadGuard<dyn Instance> {
		RwLockReadGuard::map(self.inner.component.read(), |c| {
			c.as_ref().unwrap().as_ref()
		})
	}

	pub fn component_mut(&self) -> MappedRwLockWriteGuard<dyn Instance> {
		RwLockWriteGuard::map(self.inner.component.write(), |c| {
			c.as_mut().unwrap().as_mut()
		})
	}

	pub fn def(&self) -> &ViewDef {
		&self.inner.def
	}

	pub fn element(&self) -> &Element {
		&self.inner.def.element
	}

	pub fn name(&self) -> &'static str {
		self.with_instance(|c| c.name())
	}

	pub fn level(&self) -> usize {
		self.inner.def.level
	}

	pub fn downgrade(instance: &View) -> WeakView {
		WeakView {
			inner: Arc::downgrade(&instance.inner),
		}
	}

	#[allow(dead_code)]
	pub(crate) fn add_child(&self, child: View) {
		self.write().children.push(child)
	}

	fn read(&self) -> RwLockReadGuard<ViewMut> {
		self.inner.state.read()
	}

	fn write(&self) -> RwLockWriteGuard<ViewMut> {
		self.inner.state.write()
	}

	pub fn state(&self) -> MappedRwLockReadGuard<Box<dyn Any>> {
		RwLockReadGuard::map(self.inner.state.read(), |state| {
			state.target.as_ref().unwrap()
		})
	}

	pub fn state_mut(&self) -> MappedRwLockWriteGuard<Box<dyn Any>> {
		RwLockWriteGuard::map(self.inner.state.write(), |state| {
			state.target.as_mut().unwrap()
		})
	}

	pub fn with_state<F, R>(&self, handler: F) -> R
	where
		F: FnOnce(&mut Option<Box<dyn Any>>) -> R,
	{
		(handler)(&mut self.write().target)
	}

	pub fn with_instance<F, R>(&self, handle: F) -> R
	where
		F: FnOnce(&mut dyn Instance) -> R,
	{
		let mut component = self.component_mut();
		let res = (handle)(&mut *component);
		std::mem::drop(component);
		res
	}

	pub fn render_child(
		&self,
		element: Element,
		arg: Option<Box<dyn Any>>,
	) -> Result<View, RuvieError> {
		let instance = View::new(ViewDef {
			runtime: self.inner.def.runtime.clone(),
			level: self.inner.def.level + 1,
			parent: Some(View::downgrade(&self)),
			element,
		});
		instance.perform_render(arg)?;
		Ok(instance)
	}

	pub(crate) fn render(&self, eval: &EvalContext) {
		let children = self.with_instance(|component| {
			component.render(&Render {
				eval,
				children: &self.inner.def.element.children(),
			})
		});

		self.write().rendered_tree = children;
	}

	pub(crate) fn perform_render(&self, arg: Option<Box<dyn Any>>) -> Result<(), RuvieError> {
		self.inner.render_rx.update();
		self.write().is_render_scheduled = false;
		self.mount(arg)?;
		self.perform_update()?;

		Ok(())
	}

	fn add_reaction(&self, handler: ReactionHandler) {
		let rx = Tracker::new();
		Tracker::set_eval(
			&rx,
			Arc::new(UpdateReaction {
				handler,
				view: View::downgrade(&self),
				rx: Tracker::downgrade(&rx),
			}),
		);

		rx.autorun();

		let mut mut_self = self.write();
		mut_self.stale_reactions.insert(Tracker::downgrade(&rx));
		mut_self.all_reactions.push(rx);
	}

	pub(crate) fn mount(&self, arg: Option<Box<dyn Any>>) -> Result<(), RuvieError> {
		let tree = self.write().rendered_tree.take();
		let mut ctx = Mount {
			view: self.clone(),
			children: vec![],
			reactions: vec![],
			tree,
		};

		self.inner.def.runtime.platform.mount(self, &mut ctx, arg)?;

		let Mount {
			children,
			reactions,
			..
		} = ctx;

		self.write().children = ManuallyDrop::new(children);

		for reaction in reactions {
			self.add_reaction(reaction)
		}

		if let Some(refr) = self.inner.def.element.reference() {
			refr.apply(View::downgrade(&self))
		}

		self.after_render();

		Ok(())
	}

	pub fn reference<K: Hash>(&self, reference: &K) -> Reference {
		Reference {
			parent: View::downgrade(&self),
			id: fxhash::hash64(&reference),
		}
	}

	pub fn register_reference<K: Hash>(&self, id: &K, inst: WeakView) {
		self.write().references.insert(fxhash::hash64(id), inst);
	}

	pub(crate) fn perform_update(&self) -> Result<(), RuvieError> {
		let updated = {
			let mut state = self.write();
			state.is_update_scheduled = false;
			std::mem::replace(&mut state.stale_reactions, HashSet::new())
		};

		for rx in updated {
			let rx = rx.upgrade();
			if let Some(rx) = rx {
				rx.update();
				let res = std::mem::replace(&mut self.write().update_res, Ok(()));
				if res.is_err() {
					return res;
				}
			}
		}

		Ok(())
	}

	pub(crate) fn schedule_render(&self) {
		if self.read().is_render_scheduled {
			return;
		}

		self.write().is_render_scheduled = true;
		self.inner
			.def
			.runtime
			.schedule_render(View::downgrade(&self));
	}

	pub(crate) fn schedule_update(&self) {
		if self.read().is_update_scheduled {
			return;
		}

		self.write().is_update_scheduled = true;
		self.inner
			.def
			.runtime
			.schedule_update(View::downgrade(&self));
	}

	pub(crate) fn after_render(&self) {
		let mut component = self.component_mut();
		let mut ctx = AfterRender { reactions: vec![] };
		component.after_render(&mut ctx);
		for reaction in ctx.reactions {
			self.add_reaction(reaction)
		}
	}

	#[allow(dead_code)]
	pub(crate) fn before_unmount(&mut self) {
		self.component_mut().before_unmount()
	}

	#[allow(dead_code)]
	pub(crate) fn get<K: Hash>(&self, refr: &K) -> Option<View> {
		let state = self.read();
		let res = state.references.get(&fxhash::hash64(refr));
		res.and_then(|inst| inst.upgrade())
	}
}

struct RenderReaction {
	instance: WeakView,
}

impl Evaluation for RenderReaction {
	fn eval(&self, ctx: &EvalContext) -> u64 {
		if let Some(i) = self.instance.upgrade() {
			i.render(ctx);
		} else {
			unreachable!()
		}
		0
	}

	fn on_reaction(&self) {
		if let Some(instance) = self.instance.upgrade() {
			// FIXME triggers several times, need to optimize in observe
			instance.schedule_render()
		} else {
			unreachable!()
		}
	}

	fn is_scheduled(&self) -> bool {
		true
	}
}

pub struct UpdateReaction {
	pub view: WeakView,
	pub rx: WeakTracker,
	pub handler: ReactionHandler,
}

impl Evaluation for UpdateReaction {
	fn is_scheduled(&self) -> bool {
		true
	}

	fn eval(&self, eval: &EvalContext) -> u64 {
		if let Some(view) = self.view.upgrade() {
			let mut ctx = Update { eval, view: &view };
			let res = (self.handler)(&mut ctx);
			view.write().update_res = res;
		} else {
			unreachable!()
		}
		0
	}

	fn on_reaction(&self) {
		if let Some(view) = self.view.upgrade() {
			// FIXME move to instance
			view.write().stale_reactions.insert(self.rx.clone());
			// FIXME triggers several times, need to optimize in observe
			view.schedule_update()
		} else {
			unreachable!()
		}
	}
}

pub struct WeakView {
	inner: Weak<ViewShared>,
}

impl WeakView {
	pub fn upgrade(&self) -> Option<View> {
		self.inner.upgrade().map(|inner| View { inner })
	}
}

impl Clone for WeakView {
	fn clone(&self) -> Self {
		WeakView {
			inner: self.inner.clone(),
		}
	}
}
