use crate::color::Color;
use crate::rule::Rule;
use std::fmt::Display;

pub struct BackgroundColor(Color);

impl Rule for BackgroundColor {
    fn name(&self) -> &str {
        "background-color"
    }
}

impl From<Color> for BackgroundColor {
    fn from(value: Color) -> BackgroundColor {
        BackgroundColor(value)
    }
}

impl Display for BackgroundColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        self.0.fmt(f)
    }
}
