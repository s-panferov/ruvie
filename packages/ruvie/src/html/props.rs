use super::class::ClassList;
use crate::props::Prop;

#[cfg(feature = "css")]
use ruvie_css::StyleSheet;

#[cfg(feature = "web")]
pub use crate::web::props::*;
use observe::Value;

pub enum Style {
	#[cfg(feature = "css")]
	StyleSheet(Value<StyleSheet>),
	String(Value<String>),
}

impl Prop for Style {}

pub struct ContentEditable(pub Value<bool>);
impl Prop for ContentEditable {}

pub struct Class(pub Value<ClassList>);

impl Prop for Class {}

#[derive(Hash)]
pub struct Id(pub String);

impl Prop for Id {}
