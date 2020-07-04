use crate::{handler::Handler, props::Prop};
use web_sys::{InputEvent, MouseEvent};

pub enum OnClick {
	EventListener {
		capture: bool,
		handler: Handler<MouseEvent>,
	},
}

impl Prop for OnClick {}

pub enum OnBeforeInput {
	EventListener {
		capture: bool,
		handler: Handler<InputEvent>,
	},
}

impl Prop for OnBeforeInput {}
