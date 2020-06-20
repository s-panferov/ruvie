use wasm_bindgen::convert::FromWasmAbi;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Node;

use crate::handler::Handler;

pub struct WebHandler<T> {
	_closure: Closure<dyn Fn(T)>,
}

pub trait BoxedWebHandler {}

impl<T> BoxedWebHandler for WebHandler<T> {}

// FIXME too many boxing here
pub fn bind<E: FromWasmAbi>(
	handler: &Handler<E>,
	node: &Node,
	name: &str,
) -> Result<WebHandler<E>, JsValue>
where
	E: 'static,
{
	let handler = (*handler).clone();
	let closure = Closure::wrap(Box::new(move |ev: E| {
		handler.trigger(ev);
	}) as Box<dyn Fn(_)>);
	node.add_event_listener_with_callback(name, closure.as_ref().unchecked_ref())?;
	Ok(WebHandler { _closure: closure })
}
