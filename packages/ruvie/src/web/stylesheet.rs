use harsh::Harsh;
use ruvie_css::StyleSheet;

use wasm_bindgen::JsCast;

use std::{cell::RefCell, collections::BTreeSet};
use web_sys::HtmlStyleElement;

pub struct StylesRuntime {
	node: Option<HtmlStyleElement>,
	injected: BTreeSet<String>,
}

impl StylesRuntime {
	fn new() -> Self {
		StylesRuntime {
			node: None,
			injected: BTreeSet::new(),
		}
	}

	pub fn inject(&mut self, sheet: &StyleSheet) -> String {
		let encoder = Harsh::default();
		let id = encoder.encode(&[fxhash::hash64(sheet)]);

		if !self.injected.contains(&id) {
			if self.node.is_none() {
				self.init()
			}

			let node = self.node.as_ref().unwrap();
			let window = web_sys::window().expect("no global `window` exists");
			let document = window.document().expect("should have a document on window");

			let text = document.create_text_node(&format!(".{} {{ {} }}", id, sheet.to_string()));
			node.append_child(&text).unwrap();

			self.injected.insert(id.clone());
		}

		id
	}

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

std::thread_local! {
  /// This is an example for using doc comment attributes
  pub static STYLES: RefCell<StylesRuntime> = {
	  RefCell::new(StylesRuntime::new())
  };
}
