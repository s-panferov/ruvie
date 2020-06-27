use ruvie::prelude::*;
use ruvie::{
	web::{elem::Div, Web},
	Children,
};

use ruvie::context::Render;
use std::sync::Arc;
use wasm_bindgen::{prelude::*, JsValue};

fn button(ctx: Render) -> Children {
	Div::default().children(ctx.children.clone()).into()
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
	console_error_panic_hook::set_once();

	let window = web_sys::window().expect("no global `window` exists");
	let document = window.document().expect("should have a document on window");
	let body = document.body().expect("document should have a body");

	let rt = ruvie::Runtime::new(Arc::new(Web));

	// let view = rt.render(app, Box::new(Cursor::beginning_of(&body)?));
	// Box::leak(Box::new(view));

	Ok(())
}
