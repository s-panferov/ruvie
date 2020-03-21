use crate::global::Auto;
use crate::length::Length;
use crate::percentage::Percentage;
use crate::rule::Rule;
use std::fmt::Display;

pub enum Left {
    Length(Length),
    Percentage(Percentage),
    Auto(Auto),
}

impl Display for Left {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Left::Length(v) => write!(f, "{}", v),
            Left::Percentage(v) => write!(f, "{}", v),
            Left::Auto(v) => write!(f, "{}", v),
        }
    }
}

impl From<Length> for Left {
    fn from(value: Length) -> Self {
        Left::Length(value)
    }
}

impl From<Percentage> for Left {
    fn from(value: Percentage) -> Self {
        Left::Percentage(value)
    }
}

impl From<Auto> for Left {
    fn from(value: Auto) -> Self {
        Left::Auto(value)
    }
}

impl Rule for Left {
    fn name(&self) -> &str {
        "left"
    }
}
