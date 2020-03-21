use crate::global::Auto;
use crate::length::Length;
use crate::percentage::Percentage;
use crate::rule::Rule;
use std::fmt::Display;

pub enum Height {
    Length(Length),
    Percentage(Percentage),
    Auto(Auto),
}

impl Display for Height {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Height::Length(v) => write!(f, "{}", v),
            Height::Percentage(v) => write!(f, "{}", v),
            Height::Auto(v) => write!(f, "{}", v),
        }
    }
}

impl From<Length> for Height {
    fn from(value: Length) -> Self {
        Height::Length(value)
    }
}

impl From<Percentage> for Height {
    fn from(value: Percentage) -> Self {
        Height::Percentage(value)
    }
}

impl From<Auto> for Height {
    fn from(value: Auto) -> Self {
        Height::Auto(value)
    }
}

impl Rule for Height {
    fn name(&self) -> &str {
        "height"
    }
}
