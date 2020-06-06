use std::{fmt::Display, sync::Arc};

use crate::{event::Event, props::Prop};

use observe::Value;
use rustcss::StyleSheet;
use web_sys::{InputEvent, MouseEvent};

#[derive(Hash)]
pub struct Style;

impl Prop for Style {
	type Value = Value<Option<Arc<StyleSheet>>>;
}

#[derive(Hash)]
pub struct OnClick {
	pub capture: bool,
}

impl Prop for OnClick {
	type Value = Event<MouseEvent>;
}

#[derive(Hash)]
pub struct OnBeforeInput;

impl Prop for OnBeforeInput {
	type Value = Event<InputEvent>;
}

#[derive(Hash)]
pub struct ContentEditable;

impl Prop for ContentEditable {
	type Value = Value<Option<bool>>;
}

#[derive(Hash)]
pub struct Class;

impl Prop for Class {
	type Value = Value<Option<ClassList>>;
}

#[derive(Hash)]
pub struct Id;

impl Prop for Id {
	type Value = String;
}

#[derive(Debug, Hash)]
pub struct ClassList {
	pub classes: Vec<String>,
}

impl ClassList {
	pub fn new(classes: Vec<String>) -> ClassList {
		ClassList { classes }
	}
}

impl From<Vec<String>> for ClassList {
	fn from(classes: Vec<String>) -> Self {
		ClassList { classes }
	}
}

impl Display for ClassList {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for cls in &self.classes {
			write!(f, "{}", cls)?
		}
		Ok(())
	}
}
