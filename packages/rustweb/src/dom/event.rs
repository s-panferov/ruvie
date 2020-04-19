use observe::{transaction, Timed};

use wasm_bindgen::convert::FromWasmAbi;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Node;

use crate::event::Event;

pub struct EventHandler<T> {
    _closure: Closure<dyn Fn(T)>,
}

pub trait BoxedHandler {}

impl<T> BoxedHandler for EventHandler<T> {}

pub fn bind<E: FromWasmAbi>(
    event: Event<E>,
    node: Node,
    name: &str,
) -> Result<EventHandler<E>, JsValue>
where
    E: 'static,
{
    let closure = Closure::wrap(Box::new(move |ev: E| {
        transaction(None, |tx| event.set(tx, Some(Timed::new(ev))))
    }) as Box<dyn Fn(_)>);
    node.add_event_listener_with_callback(name, closure.as_ref().unchecked_ref())?;
    Ok(EventHandler { _closure: closure })
}
