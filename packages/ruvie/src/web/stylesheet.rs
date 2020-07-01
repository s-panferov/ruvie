use harsh::Harsh;
use ruvie_css::StyleSheet;

use wasm_bindgen::JsCast;

use std::{cell::RefCell, collections::BTreeSet};
use web_sys::HtmlStyleElement;

pub struct HtmlStyleElementRuntime {
	body: RefCell<HtmlStyleElementRuntimeMut>,
}

struct HtmlStyleElementRuntimeMut {
	node: Option<HtmlStyleElement>,
	injected: BTreeSet<String>,
}

impl HtmlStyleElementRuntimeMut {
	fn init(&mut self) {
		let window = web_sys::window().expect("no global `window` exists");
		let document = window.document().expect("should have a document on window");
		let style = document.get_element_by_id("styles");

		match style {
			Some(style) => self.node = Some(style.unchecked_into()),
			None => {
				let head = document.head().expect("document should have a head");
				let style = document.create_element("style").unwrap();
				head.append_child(&style).unwrap();
				self.node = Some(style.unchecked_into());
			}
		}
	}
}

impl HtmlStyleElementRuntime {
	pub fn new() -> Self {
		HtmlStyleElementRuntime {
			body: RefCell::new(HtmlStyleElementRuntimeMut {
				node: None,
				injected: BTreeSet::new(),
			}),
		}
	}
}

impl crate::html::StyleRuntime for HtmlStyleElementRuntime {
	fn inject(&self, sheet: &StyleSheet, f: &mut std::fmt::Formatter<'_>) {
		let encoder = Harsh::default();
		let id = encoder.encode(&[fxhash::hash64(sheet)]);
		if !self.body.borrow().injected.contains(&id) {
			let mut mut_body = self.body.borrow_mut();
			if mut_body.node.is_none() {
				mut_body.init()
			}

			let node = mut_body.node.as_ref().unwrap();
			let window = web_sys::window().expect("no global `window` exists");
			let document = window.document().expect("should have a document on window");

			let text = document.create_text_node(&format!(".{} {{ {} }}", id, sheet.to_string()));
			node.append_child(&text).unwrap();

			mut_body.injected.insert(id.clone());
		}
		write!(f, "{}", id).unwrap()
	}
}
