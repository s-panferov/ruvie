#[cfg(feature = "css")]
use ruvie_css::StyleSheet;

use std::{
	borrow::Cow,
	fmt::{Display, Formatter},
};

#[derive(Debug, Hash, Clone)]
pub enum ClassItem {
	String(Cow<'static, str>),

	#[cfg(feature = "css")]
	StyleSheet(Cow<'static, StyleSheet>),
}

pub trait StyleRuntime {
	#[cfg(feature = "css")]
	fn inject(&self, style: &StyleSheet, f: &mut Formatter<'_>);
}

#[derive(Hash, Debug)]
pub struct ClassList {
	pub classes: Vec<ClassItem>,
}

pub struct ClassListFormatter<'a> {
	list: &'a ClassList,
	runtime: &'a dyn StyleRuntime,
}

impl<'a> Display for ClassListFormatter<'a> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		for cls in &self.list.classes {
			match cls {
				ClassItem::String(class) => write!(f, "{}", class)?,
				#[cfg(feature = "css")]
				ClassItem::StyleSheet(sheet) => self.runtime.inject(&sheet, f),
			}
		}
		Ok(())
	}
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

	pub fn format(&self, runtime: &dyn StyleRuntime) -> String {
		ClassListFormatter {
			list: self,
			runtime,
		}
		.to_string()
	}
}

#[cfg(feature = "css")]
impl From<StyleSheet> for ClassItem {
	fn from(v: StyleSheet) -> Self {
		ClassItem::StyleSheet(Cow::Owned(v))
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
		let mut list = $crate::html::ClassList::new(vec![]);

		$(
			$crate::cx_inner!(list, $class $(=> if $test)* );
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
