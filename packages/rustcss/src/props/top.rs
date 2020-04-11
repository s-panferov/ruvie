use crate::global::Auto;
use crate::length::Length;
use crate::percentage::Percentage;
use crate::rule::Rule;
use std::fmt::Display;

pub enum Top {
    Length(Length),
    Percentage(Percentage),
    Auto(Auto),
}

impl Display for Top {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Top::Length(v) => write!(f, "{}", v),
            Top::Percentage(v) => write!(f, "{}", v),
            Top::Auto(v) => write!(f, "{}", v),
        }
    }
}

impl From<Length> for Top {
    fn from(value: Length) -> Self {
        Top::Length(value)
    }
}

impl From<Percentage> for Top {
    fn from(value: Percentage) -> Self {
        Top::Percentage(value)
    }
}

impl From<Auto> for Top {
    fn from(value: Auto) -> Self {
        Top::Auto(value)
    }
}

impl Rule for Top {
    fn name(&self) -> &str {
        "top"
    }
}
