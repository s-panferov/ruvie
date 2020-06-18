use rustcss::StyleSheet;
use std::{borrow::Cow, fmt::Display};

#[derive(Debug, Hash, Clone)]
pub enum ClassItem {
	String(Cow<'static, str>),
	StyleSheet(StyleSheet),
}

struct StyleSheetInject {}

impl Display for ClassItem {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ClassItem::String(s) => write!(f, "{}", s),
			ClassItem::StyleSheet(s) => {
				let class_name = super::stylesheet::STYLES.with(|rt| {
					let mut rt = rt.borrow_mut();
					rt.inject(s)
				});
				write!(f, "{}", class_name)
			}
		}
	}
}

#[derive(Hash, Debug)]
pub struct ClassList {
	pub classes: Vec<ClassItem>,
}

impl ClassList {
	pub fn new(classes: Vec<ClassItem>) -> ClassList {
		ClassList { classes }
	}

	pub fn empty() -> ClassList {
		ClassList { classes: vec![] }
	}

	pub fn push<I: Into<ClassItem>>(&mut self, class: I) {
		self.classes.push(class.into())
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

impl From<StyleSheet> for ClassItem {
	fn from(v: StyleSheet) -> Self {
		ClassItem::StyleSheet(v)
	}
}

impl From<&'static str> for ClassItem {
	fn from(v: &'static str) -> Self {
		ClassItem::String(Cow::Borrowed(v))
	}
}

impl From<String> for ClassItem {
	fn from(v: String) -> Self {
		ClassItem::String(Cow::Owned(v))
	}
}

#[macro_export]
macro_rules! cx {
	($($class:expr $(=> if $test:expr)?),*) => [{
		let mut list = ClassList::new(vec![]);

		$(
			rustweb::cx_inner!(list, $class $(=> if $test)* );
		)*

		list
	}];
}

#[macro_export]
macro_rules! cx_inner {
	($list:ident, $class:expr => if $test:expr) => {
		if $test {
			$list.push($class);
			}
	};
	($list:ident, $class:expr) => {
		$list.push($class);
	};
}

// impl From<Value<StyleSheet>> for ClassList {
// 	// fn from(v: StyleSheet) -> Self {
// 		ClassList {
// 			classes: vec![ClassItem::StyleSheet(v)],
// 		}
// 	}
// }
