use std::cell::{Ref, RefCell, RefMut};
use std::{
	collections::{HashMap, HashSet},
	hash::Hash,
	marker::PhantomData,
	mem::ManuallyDrop,
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
	target::Target,
};

use crate::element::Element;

pub struct ViewDef<T: Target> {
	pub runtime: Runtime<T>,
	pub parent: Option<WeakView<T>>,
	pub element: Element<T>,
	pub level: usize,
}

pub struct View<T: Target + ?Sized> {
	inner: Arc<ViewInner<T>>,
}

impl<T> Clone for View<T>
where
	T: Target,
{
	fn clone(&self) -> Self {
		View {
			inner: self.inner.clone(),
		}
	}
}

pub struct ViewInner<T: Target> {
	pub(crate) def: ViewDef<T>,
	render_rx: Tracker,
	pub component: RwLock<Option<Box<dyn Instance<T>>>>,
	state: RwLock<ViewMut<T>>,
}

pub struct ViewMut<T: Target> {
	rendered_tree: Children<T>,
	pub children: ManuallyDrop<Vec<View<T>>>,
	is_render_scheduled: bool,
	is_update_scheduled: bool,
	update_res: Result<(), T::Error>,
	all_update_reactions: Vec<Tracker>,
	invalidated_updates: HashSet<WeakTracker>,
	references: HashMap<u64, WeakView<T>>,
	pub(crate) target: ManuallyDrop<Option<T::State>>,
}

pub type ReactionCallback<T> =
	Box<dyn for<'a> Fn(&'a View<T>, &'a mut Update<'a, T>) -> Result<(), <T as Target>::Error>>;

impl<T> Drop for ViewMut<T>
where
	T: Target,
{
	fn drop(&mut self) {
		unsafe {
			let _ = ManuallyDrop::drop(&mut self.target);
			let _ = ManuallyDrop::drop(&mut self.children);
		}
	}
}

impl<T: Target> View<T> {
	pub fn new(spec: ViewDef<T>) -> Self {
		let state = ViewMut {
			children: ManuallyDrop::new(vec![]),
			rendered_tree: None.into(),
			all_update_reactions: vec![],
			invalidated_updates: HashSet::new(),
			update_res: Ok(()),
			is_render_scheduled: false,
			is_update_scheduled: false,
			references: HashMap::new(),
			target: ManuallyDrop::new(None),
		};

		let view = View {
			inner: Arc::new(ViewInner {
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

	pub fn component(&self) -> MappedRwLockReadGuard<dyn Instance<T>> {
		RwLockReadGuard::map(self.inner.component.read(), |c| {
			c.as_ref().unwrap().as_ref()
		})
	}

	pub fn component_mut(&self) -> MappedRwLockWriteGuard<dyn Instance<T>> {
		RwLockWriteGuard::map(self.inner.component.write(), |c| {
			c.as_mut().unwrap().as_mut()
		})
	}

	pub fn element(&self) -> &Element<T> {
		&self.inner.def.element
	}

	pub fn name(&self) -> &'static str {
		self.with_instance(|c| c.name())
	}

	pub fn level(&self) -> usize {
		self.inner.def.level
	}

	pub fn downgrade(instance: &View<T>) -> WeakView<T> {
		WeakView {
			inner: Arc::downgrade(&instance.inner),
		}
	}

	pub(crate) fn add_child(&self, child: View<T>) {
		self.state_mut().children.push(child)
	}

	fn state(&self) -> RwLockReadGuard<ViewMut<T>> {
		self.inner.state.read()
	}

	fn state_mut(&self) -> RwLockWriteGuard<ViewMut<T>> {
		self.inner.state.write()
	}

	pub fn platform(&self) -> MappedRwLockReadGuard<T::State> {
		RwLockReadGuard::map(self.inner.state.read(), |state| {
			state.target.as_ref().unwrap()
		})
	}

	pub fn platform_mut(&self) -> MappedRwLockWriteGuard<T::State> {
		RwLockWriteGuard::map(self.inner.state.write(), |state| {
			state.target.as_mut().unwrap()
		})
	}

	pub fn with_state<F, R>(&self, handler: F) -> R
	where
		F: FnOnce(&mut Option<T::State>) -> R,
	{
		(handler)(&mut self.state_mut().target)
	}

	pub fn with_instance<F, R>(&self, handle: F) -> R
	where
		F: FnOnce(&mut dyn Instance<T>) -> R,
	{
		let mut component = self.component_mut();
		let res = (handle)(&mut *component);
		std::mem::drop(component);
		res
	}

	pub fn render_child(
		&self,
		element: Element<T>,
		arg: Option<T::Arg>,
	) -> Result<View<T>, T::Error> {
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

		self.state_mut().rendered_tree = children;
	}

	pub(crate) fn perform_render(&self, arg: Option<T::Arg>) -> Result<(), T::Error> {
		self.inner.render_rx.update();
		self.state_mut().is_render_scheduled = false;
		self.mount(arg)?;
		self.perform_update()?;

		Ok(())
	}

	fn add_reaction(&self, handler: ReactionCallback<T>) {
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

		self.state_mut()
			.invalidated_updates
			.insert(Tracker::downgrade(&rx));

		self.state_mut().all_update_reactions.push(rx);
	}

	pub(crate) fn mount(&self, arg: Option<T::Arg>) -> Result<(), T::Error> {
		let tree = self.state_mut().rendered_tree.take();
		let ctx = Mount {
			view: self.clone(),
			children: vec![],
			reactions: vec![],
			tree,
		};

		let ctx = T::mount(self, ctx, arg)?;
		let Mount {
			children,
			reactions,
			..
		} = ctx;

		self.state_mut().children = ManuallyDrop::new(children);
		for reaction in reactions {
			self.add_reaction(reaction)
		}

		if let Some(refr) = self.inner.def.element.reference() {
			refr.apply(View::downgrade(&self))
		}

		self.after_render();

		Ok(())
	}

	pub fn reference<K: Hash>(&self, reference: &K) -> Reference<T> {
		Reference {
			parent: View::downgrade(&self),
			id: fxhash::hash64(&reference),
		}
	}

	pub fn register_reference<K: Hash>(&self, id: &K, inst: WeakView<T>) {
		self.state_mut().references.insert(fxhash::hash64(id), inst);
	}

	pub(crate) fn perform_update(&self) -> Result<(), T::Error> {
		let updated = {
			let mut state = self.state_mut();
			state.is_update_scheduled = false;
			std::mem::replace(&mut state.invalidated_updates, HashSet::new())
		};

		for rx in updated {
			let rx = rx.upgrade();
			if let Some(rx) = rx {
				rx.update();
				let res = std::mem::replace(&mut self.state_mut().update_res, Ok(()));
				if res.is_err() {
					return res;
				}
			}
		}

		Ok(())
	}

	pub(crate) fn schedule_render(&self) {
		if self.state().is_render_scheduled {
			return;
		}

		self.state_mut().is_render_scheduled = true;
		self.inner
			.def
			.runtime
			.schedule_render(View::downgrade(&self));
	}

	pub(crate) fn schedule_update(&self) {
		if self.state().is_update_scheduled {
			return;
		}

		self.state_mut().is_update_scheduled = true;
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

	pub(crate) fn before_unmount(&mut self) {
		self.component_mut().before_unmount()
	}

	pub(crate) fn get<K: Hash>(&self, refr: &K) -> Option<View<T>> {
		let state = self.state();
		let res = state.references.get(&fxhash::hash64(refr));
		res.and_then(|inst| inst.upgrade())
	}
}

struct RenderReaction<T>
where
	T: Target,
{
	instance: WeakView<T>,
}

impl<T> Evaluation for RenderReaction<T>
where
	T: Target,
{
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

pub struct UpdateReaction<T>
where
	T: Target,
{
	pub view: WeakView<T>,
	pub rx: WeakTracker,
	pub handler: Box<dyn for<'a> Fn(&'a View<T>, &'a mut Update<'a, T>) -> Result<(), T::Error>>,
}

impl<T> Evaluation for UpdateReaction<T>
where
	T: Target,
{
	fn is_scheduled(&self) -> bool {
		true
	}

	fn eval(&self, eval: &EvalContext) -> u64 {
		if let Some(view) = self.view.upgrade() {
			let mut ctx = Update {
				eval,
				view: &view,
				_t: PhantomData,
			};
			let res = (self.handler)(&view, &mut ctx);
			view.state_mut().update_res = res;
		} else {
			unreachable!()
		}
		0
	}

	fn on_reaction(&self) {
		if let Some(view) = self.view.upgrade() {
			// FIXME move to instance
			view.state_mut().invalidated_updates.insert(self.rx.clone());
			// FIXME triggers several times, need to optimize in observe
			view.schedule_update()
		} else {
			unreachable!()
		}
	}
}

pub struct WeakView<T>
where
	T: Target,
{
	inner: Weak<ViewInner<T>>,
}

impl<T> WeakView<T>
where
	T: Target,
{
	pub fn upgrade(&self) -> Option<View<T>> {
		self.inner.upgrade().map(|inner| View { inner })
	}
}

impl<T> Clone for WeakView<T>
where
	T: Target,
{
	fn clone(&self) -> Self {
		WeakView {
			inner: self.inner.clone(),
		}
	}
}
