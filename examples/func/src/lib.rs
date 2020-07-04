use ruvie::prelude::*;
use ruvie::{
	html::el::div,
	html::props,
	web::{Cursor, Web},
	Children,
};

use ruvie::{context::Render, Handler};
use wasm_bindgen::{prelude::*, JsValue};

fn button(ctx: &Render) -> Children {
	div::prop(props::OnClick::EventListener {
		capture: false,
		handler: Handler::new(|ev| {}),
	})
	.children(ctx.children.clone())
	.into()
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
	console_error_panic_hook::set_once();

	let window = web_sys::window().expect("no global `window` exists");
	let document = window.document().expect("should have a document on window");
	let body = document.body().expect("document should have a body");

	let rt = ruvie::Runtime::new(Web::new());

	let view = rt.render(button.default(), Box::new(Cursor::beginning_of(&body)?));

	Box::leak(Box::new(view));

	Ok(())
}
