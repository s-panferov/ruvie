use std::fmt::Display;

use crate::{event::Event, props::Prop};

use observe::local::Value;
use rustcss::StyleSheet;
use web_sys::MouseEvent;

#[derive(Default, Debug)]
pub struct HtmlProps {
    pub style: Value<Option<StyleSheet>>,
    pub class: Value<Option<ClassList>>,
    pub contenteditable: Value<Option<bool>>,
    pub on_click: Option<Event<MouseEvent>>,
}

#[derive(Hash)]
pub struct Style;

impl Prop for Style {
    type Value = Value<Option<StyleSheet>>;
}

#[derive(Hash)]
pub struct OnClick {
    pub capture: bool,
}

impl Prop for OnClick {
    type Value = Event<MouseEvent>;
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

#[derive(Debug, Hash)]
pub struct ClassList {
    pub classes: Vec<String>,
}

impl ClassList {
    pub fn new(classes: Vec<String>) -> ClassList {
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
