use std::sync::Arc;
use std::task::Poll;

use wasm_bindgen::{prelude::*, JsCast, JsValue};

use observe::{transaction, Computed, MutObservable, Observable, Value, Var};

use ruvie::{
	context::Render,
	contrib::list::{List, ListProps},
	prelude::*,
	web::{elem::Div, Class, ClassList, Cursor, Style, Web},
	Children, Component, Constructor, Element, Scope,
};

use store::AppStore;
use store::{AppProps, Task, Theme};

mod api;
mod error;
mod store;

fn button() -> Children {
	Children::from(None)
}

struct App {
	store: Arc<AppStore>,

	#[allow(unused)]
	scope: Scope<Self>,
}

impl App {
	fn render_list(&mut self) -> Element {
		List::with_props(ListProps {
			list: Value::from(self.store.tasks.clone()),
			hint: Default::default(),
			item: Arc::new(move |_, task| Div::default().child(task.title.clone()).build()),
		})
		.build()
	}
}

impl Constructor for App {
	fn create(props: Self::Props, scope: Scope<Self>) -> Self {
		App {
			store: props,
			scope,
		}
	}
}

impl Component for App {
	type Props = Arc<AppStore>;

	fn render(&mut self, _ctx: &Render) -> Children {
		let store = &self.store;
		let payload = store.data.clone();

		Div::prop(Style, &store.style)
			.prop(Class, ruvie::cx!("test"))
			.scope(move |_ctx| {
				let payload = payload.clone();
				Value::from(Computed::new(move |eval| match &*payload.get(eval) {
					Poll::Ready(_v) => format!("DATA: {:?}", _v),
					Poll::Pending => String::from("Loading"),
				}))
			})
			.scope(|_| button())
			.child(self.render_list())
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

	let app = App::with_props(store.clone()).build();
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

	let rt = ruvie::Runtime::new(Web);

	let view = rt.render(app, Box::new(Cursor::beginning_of(&body)?));

	Box::leak(Box::new(view));

	Ok(())
}
