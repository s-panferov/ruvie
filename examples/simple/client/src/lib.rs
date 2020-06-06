#![allow(non_snake_case)]

use std::sync::Arc;
use std::task::Poll;

use wasm_bindgen::{prelude::*, JsCast, JsValue};

use observe::{transaction, Computed, MutObservable, Observable, Value, Var};

use rustweb::{
	context::Render,
	contrib::list::{List, ListProps},
	prelude::*,
	web::{elem::Div, Class, ClassList, Cursor, Style, Web},
	Children, Html, Scope, Target,
};

use store::AppStore;
use store::{AppProps, Task, Theme};

mod api;
mod error;
mod store;

struct App<T>
where
	T: Target<Realm = Html>,
{
	store: Arc<AppStore>,
	scope: Scope<Self, T>,
}

impl<T: Target<Realm = Html>> Component<T> for App<T> {
	type Props = Arc<AppStore>;

	fn create(props: Self::Props, scope: Scope<Self, T>) -> Self {
		App {
			store: props,
			scope,
		}
	}

	fn render(&mut self, _ctx: &Render<T>) -> Children<T> {
		let store = &self.store;
		let payload = store.data.clone();

		Div::<T>::prop(Style, &store.style)
			.prop(Class, ClassList::from(vec![String::from("test")]))
			.scope(move |_ctx| {
				let payload = payload.clone();
				Value::from(Computed::new(move |eval| match &*payload.get(eval) {
					Poll::Ready(_v) => format!("DATA: {:?}", _v),
					Poll::Pending => String::from("Loading"),
				}))
			})
			.child(
				List::with_props(ListProps {
					list: Value::from(store.tasks.clone()),
					hint: Default::default(),
					item: Arc::new(move |_, task| Div::default().child(task.title.clone()).build()),
				})
				.build(),
			)
			.into()
	}
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
	console_error_panic_hook::set_once();

	let x = Var::new(0i32);
	let y = Var::new(0i32);
	let theme = Var::new(Theme::Square);

	let store = AppStore::new(AppProps {
		theme: theme.clone().into(),
		x: x.clone().into(),
		y: y.clone().into(),
	});

	let app = App::<Web>::with_props(store.clone()).build();
	let window = web_sys::window().expect("no global `window` exists");
	let document = window.document().expect("should have a document on window");
	let body = document.body().expect("document should have a body");

	let mousemove = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
		transaction(None, |tx| {
			x.set(tx, event.x());
			y.set(tx, event.y());
		});
	}) as Box<dyn FnMut(_)>);

	let mut id = 1;

	let click = Closure::wrap(Box::new(move |_ev: web_sys::MouseEvent| {
		transaction(None, |tx| {
			let mut tasks = (**store.tasks.once()).clone();
			id += 1;
			let name = format!("Task {}", id);
			tasks.insert(
				id,
				Task {
					id,
					title: name,
					completed: false,
				},
			);
			id += 1;
			let name = format!("Task {}", id);
			tasks.insert(
				id,
				Task {
					id,
					title: name,
					completed: false,
				},
			);
			store.tasks.set(tx, Arc::new(tasks))
		});
		transaction(None, |tx| {
			let current = theme.once();
			let next_theme = match *current {
				Theme::Circle => Theme::Square,
				Theme::Square => Theme::Circle,
			};
			std::mem::drop(current);
			theme.set(tx, next_theme);
		});
	}) as Box<dyn FnMut(_)>);

	document.add_event_listener_with_callback("mousemove", mousemove.as_ref().unchecked_ref())?;
	document.add_event_listener_with_callback("click", click.as_ref().unchecked_ref())?;

	mousemove.forget();
	click.forget();

	let rt = rustweb::Runtime::new();

	let view = rt.render(app, Cursor::beginning_of(&body)?)?;

	Box::leak(Box::new(view));

	Ok(())
}
