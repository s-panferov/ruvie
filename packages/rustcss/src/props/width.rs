use std::fmt::Display;

use crate::global::Auto;
use crate::length::Length;
use crate::percentage::Percentage;
use crate::rule::Rule;

pub enum Width {
    Length(Length),
    Percentage(Percentage),
    Auto(Auto),
}

impl Display for Width {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Width::Length(v) => write!(f, "{}", v),
            Width::Percentage(v) => write!(f, "{}", v),
            Width::Auto(v) => write!(f, "{}", v),
        }
    }
}

impl From<Length> for Width {
    fn from(value: Length) -> Self {
        Width::Length(value)
    }
}

impl From<Percentage> for Width {
    fn from(value: Percentage) -> Self {
        Width::Percentage(value)
    }
}

impl From<Auto> for Width {
    fn from(value: Auto) -> Self {
        Width::Auto(value)
    }
}

impl Rule for Width {
    fn name(&self) -> &str {
        "width"
    }
}
