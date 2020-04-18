use crate::color::Color;
use crate::rule::{Attribute, ValueFor};

pub struct BackgroundColor;

impl Attribute for BackgroundColor {
    const NAME: &'static str = "background-color";
}

impl ValueFor<BackgroundColor> for Color {}
