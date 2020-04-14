use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::hash::Hasher;

use web_sys::Element;

use crate::{
    props::{BackgroundColor, Height, Left, Position, Property, Top, Width},
    rule::Rule,
};

pub struct StyleSheet {
    name: Option<&'static str>,
    rules: Vec<Property>,
}

impl StyleSheet {
    pub fn new() -> StyleSheet {
        StyleSheet {
            name: None,
            rules: Vec::new(),
        }
    }

    pub fn with_name(&mut self, name: &'static str) -> &mut Self {
        self.name = Some(name);
        self
    }

    pub fn create_style(&self) -> Element {
        let window = web_sys::window().expect("Window");
        let document = window.document().expect("Document");
        let style = document.create_element("style").expect("Style");
        let text = document.create_text_node(&self.to_string());
        style.append_child(&text).unwrap();
        style
    }

    pub fn width<T: Into<Width>>(&mut self, width: T) -> &mut Self {
        self.rules.push(width.into().into());
        self
    }

    pub fn position(&mut self, value: Position) -> &mut Self {
        self.rules.push(value.into());
        self
    }

    pub fn left<T: Into<Left>>(&mut self, left: T) -> &mut Self {
        self.rules.push(left.into().into());
        self
    }

    pub fn top<T: Into<Top>>(&mut self, top: T) -> &mut Self {
        self.rules.push(top.into().into());
        self
    }

    pub fn background_color<T: Into<BackgroundColor>>(&mut self, value: T) -> &mut Self {
        self.rules.push(value.into().into());
        self
    }

    pub fn height<T: Into<Height>>(&mut self, height: T) -> &mut Self {
        self.rules.push(height.into().into());
        self
    }
}

impl Debug for StyleSheet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        Display::fmt(self, f)
    }
}

impl Display for StyleSheet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        for r in &self.rules {
            write!(f, "{}: {};", r.name(), r)?
        }

        Ok(())
    }
}

impl Hash for StyleSheet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_string().hash(state)
    }
}
