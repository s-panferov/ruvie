use crate::rule::Rule;
use std::fmt::Display;

pub enum Position {
    Absolute,
    Relative,
}

impl Rule for Position {
    fn name(&self) -> &str {
        "position"
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Position::Absolute => f.write_str("absolute"),
            Position::Relative => f.write_str("relative"),
        }
    }
}
