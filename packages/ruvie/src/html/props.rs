use crate::{handler::Handler, props::Prop};

use super::class::ClassList;
use observe::Value;
use ruvie_css::StyleSheet;
use web_sys::{InputEvent, MouseEvent};

#[derive(Hash)]
pub struct Style;

impl Prop for Style {
	type Value = Value<StyleSheet>;
}

#[derive(Hash)]
pub struct OnClick {
	pub capture: bool,
}

impl OnClick {
	pub fn new() -> OnClick {
		OnClick { capture: false }
	}

	pub fn capture() -> OnClick {
		OnClick { capture: true }
	}
}

impl Prop for OnClick {
	type Value = Handler<MouseEvent>;
}

#[derive(Hash)]
pub struct OnBeforeInput;

impl Prop for OnBeforeInput {
	type Value = Handler<InputEvent>;
}

#[derive(Hash)]
pub struct ContentEditable;

impl Prop for ContentEditable {
	type Value = Value<bool>;
}

#[derive(Hash)]
pub struct Class;

impl Prop for Class {
	type Value = Value<ClassList>;
}

#[derive(Hash)]
pub struct Id;

impl Prop for Id {
	type Value = String;
}
