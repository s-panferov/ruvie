use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{prelude::*, JsCast, JsValue};

use crate::runtime::Runtime;
use crate::{
	context::Mount,
	target::{Html, Target},
	View,
};

use super::{
	cursor::Cursor,
	event::BoxedHandler,
	fragment::{FragmentBuilder, PersistedFragment, SharedPersistedFragment},
	utils, WebContext,
};
use parking_lot::MappedRwLockReadGuard;

#[derive(Clone, Debug)]
pub struct Web;

pub struct WebState {
	pub fragment: SharedPersistedFragment,
	pub handlers: Vec<Box<dyn BoxedHandler>>,
}

impl Target for Web {
	type Realm = Html;

	type Arg = Cursor;
	type Mount = WebContext;
	type Error = JsValue;
	type State = WebState;
	type Result = SharedPersistedFragment;

	fn mount_component(ctx: &mut WebContext) -> Result<(), JsValue> {
		utils::mount_children(ctx, None)
	}

	fn mount(
		view: &View<Self>,
		ctx: Mount<Web>,
		arg: Option<Cursor>,
	) -> Result<Mount<Web>, Self::Error> {
		let window = web_sys::window().expect("no global `window` exists");
		let document = window.document().expect("should have a document on window");

		let mut html = WebContext {
			doc: document,
			fragment: FragmentBuilder::new(),
			handlers: vec![],
			mount: ctx,
		};

		view.with_instance(|component| component.mount(&mut html))?;

		let WebContext {
			fragment,
			handlers,
			mount,
			doc,
			..
		} = html;

		let name = view.name();

		view.with_state(|state| {
			if state.is_none() {
				let fragment = PersistedFragment::new(doc.clone(), Some(name), fragment.children)?;
				if let Some(cursor) = arg {
					cursor.range.insert_node(&fragment.extract()?)?;
				}

				let fragment = Rc::new(RefCell::new(fragment));
				*state = Some(WebState { fragment, handlers })
			} else {
				let rt = state.as_mut().unwrap();
				rt.fragment
					.borrow_mut()
					.replace_children(&doc, fragment.children)?
			}

			Ok::<(), Self::Error>(())
		})?;

		Ok(mount)
	}

	fn schedule_tick(runtime: &Runtime<Web>) {
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
