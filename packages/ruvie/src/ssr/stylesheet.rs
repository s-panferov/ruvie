use harsh::Harsh;
use ruvie_css::StyleSheet;
use std::fmt::Write;

use std::{cell::RefCell, collections::BTreeMap};

pub struct StaticStyleRuntime {
	body: RefCell<StaticStyleRuntimeMut>,
}

struct StaticStyleRuntimeMut {
	styles: BTreeMap<String, String>,
}

impl StaticStyleRuntime {
	pub fn new() -> Self {
		StaticStyleRuntime {
			body: RefCell::new(StaticStyleRuntimeMut {
				styles: BTreeMap::new(),
			}),
		}
	}

	pub fn serialize(&self) -> String {
		let mut buf = String::new();
		for (sel, style) in self.body.borrow().styles.iter() {
			write!(buf, ".{} {{ {} }}", sel, style).unwrap()
		}

		buf
	}
}

impl crate::html::StyleRuntime for StaticStyleRuntime {
	fn inject(&self, sheet: &StyleSheet, f: &mut std::fmt::Formatter<'_>) {
		let encoder = Harsh::default();
		let style = sheet.to_string();
		let id = encoder.encode(&[fxhash::hash64(&style)]);
		if !self.body.borrow().styles.contains_key(&id) {
			let mut mut_body = self.body.borrow_mut();
			mut_body.styles.insert(id.clone(), style);
		}
		write!(f, "{}", id).unwrap()
	}
}
