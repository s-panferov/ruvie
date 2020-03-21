use crate::props::BackgroundColor;
use crate::props::Height;
use crate::props::Left;
use crate::props::Position;
use crate::props::Property;
use crate::props::Top;
use crate::props::Width;
use crate::rule::Rule;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::hash::Hasher;

pub struct StyleSheet {
    rules: Vec<Property>,
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

impl StyleSheet {
    pub fn new() -> StyleSheet {
        StyleSheet { rules: Vec::new() }
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
