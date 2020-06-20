use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::{collections::BTreeMap, hash::Hasher};

use crate::{
	props::{BackgroundColor, Height, Left, MinHeight, MinWidth, Position, Top, Width},
	rule::{Attribute, ValueFor},
};

#[derive(Clone)]
pub struct StyleSheet {
	name: Option<&'static str>,
	rules: BTreeMap<&'static str, String>,
}

impl StyleSheet {
	pub fn new() -> StyleSheet {
		StyleSheet {
			name: None,
			rules: BTreeMap::new(),
		}
	}

	pub fn with_name(mut self, name: &'static str) -> Self {
		self.name = Some(name);
		self
	}

	// pub fn create_style(&self) -> Element {
	//     let window = web_sys::window().expect("Window");
	//     let document = window.document().expect("Document");
	//     let style = document.create_element("style").expect("Style");
	//     let text = document.create_text_node(&self.to_string());
	//     style.append_child(&text).unwrap();
	//     style
	// }

	pub fn add<A: Attribute, V: ValueFor<A>>(mut self, value: V) -> Self {
		self.rules.insert(A::NAME, value.value());
		self
	}

	pub fn width<T: ValueFor<Width>>(self, value: T) -> Self {
		self.add::<Width, T>(value)
	}

	pub fn min_width<T: ValueFor<MinWidth>>(self, value: T) -> Self {
		self.add::<MinWidth, T>(value)
	}

	pub fn position<T: ValueFor<Position>>(self, value: T) -> Self {
		self.add::<Position, T>(value)
	}

	pub fn left<T: ValueFor<Left>>(self, value: T) -> Self {
		self.add::<Left, T>(value)
	}

	pub fn top<T: ValueFor<Top>>(self, value: T) -> Self {
		self.add::<Top, T>(value)
	}

	pub fn background_color<T: ValueFor<BackgroundColor>>(self, value: T) -> Self {
		self.add::<BackgroundColor, T>(value)
	}

	pub fn height<T: ValueFor<Height>>(self, value: T) -> Self {
		self.add::<Height, T>(value)
	}

	pub fn min_height<T: ValueFor<MinHeight>>(self, value: T) -> Self {
		self.add::<MinHeight, T>(value)
	}
}

impl Debug for StyleSheet {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		Display::fmt(self, f)
	}
}

impl Display for StyleSheet {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		for (k, v) in &self.rules {
			write!(f, "{}: {};", k, v)?
		}

		Ok(())
	}
}

impl Hash for StyleSheet {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.to_string().hash(state)
	}
}
