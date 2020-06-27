use std::{any::Any, cell::RefCell, rc::Rc};

use wasm_bindgen::{prelude::*, JsCast};

use crate::runtime::Runtime;
use crate::{context::Mount, error::RuvieError, target::Target, View};

use super::{
	cursor::Cursor,
	event::BoxedWebHandler,
	fragment::{FragmentBuilder, PersistedFragment, SharedPersistedFragment},
	utils, WebContext,
};

#[derive(Clone, Debug)]
pub struct Web;

pub struct WebState {
	pub fragment: SharedPersistedFragment,
	pub handlers: Vec<Box<dyn BoxedWebHandler>>,
}

impl Target for Web {
	fn mount_component(&self, ctx: &mut Mount, target: &mut dyn Any) -> Result<(), RuvieError> {
		if target.is::<WebContext>() {
			utils::mount_children(ctx, target.downcast_mut::<WebContext>().unwrap(), None)
		} else {
			Ok(()) // FIXME should be an error
		}
	}

	fn mount(
		&self,
		view: &View,
		ctx: &mut Mount,
		arg: Option<Box<dyn Any>>,
	) -> Result<(), RuvieError> {
		let cursor = arg.map(|a| a.downcast::<Cursor>().unwrap());

		let window = web_sys::window().expect("no global `window` exists");
		let document = window.document().expect("should have a document on window");

		let mut html = WebContext {
			doc: document,
			fragment: FragmentBuilder::new(),
			handlers: vec![],
		};

		view.with_instance(|component| component.mount(ctx, &mut html))?;

		let WebContext {
			fragment,
			handlers,
			doc,
			..
		} = html;

		let name = view.name();

		view.with_state(|state| {
			if state.is_none() {
				let fragment = PersistedFragment::new(doc.clone(), Some(name), fragment.children)?;
				if let Some(cursor) = cursor {
					cursor.range.insert_node(&fragment.extract()?)?;
				}

				let fragment = Rc::new(RefCell::new(fragment));
				*state = Some(Box::new(WebState { fragment, handlers }))
			} else {
				let rt = state.as_mut().unwrap().downcast_mut::<WebState>().unwrap();
				rt.fragment
					.borrow_mut()
					.replace_children(&doc, fragment.children)?
			}

			Ok::<(), RuvieError>(())
		})?;

		Ok(())
	}

	fn schedule_tick(&self, runtime: &Runtime) {
		let runtime = runtime.clone();
		let callback = Closure::once(move || runtime.tick().unwrap());

		web_sys::window()
			.unwrap()
			.request_animation_frame(callback.as_ref().unchecked_ref())
			.unwrap();

		// FIXME
		callback.forget();
	}
}
