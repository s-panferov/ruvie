use std::fmt::Display;

use crate::rule::{Attribute, ValueFor};

pub struct Position;

impl Attribute for Position {
    const NAME: &'static str = "position";
}

pub enum PositionType {
    Absolute,
    Relative,
}

impl Display for PositionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            PositionType::Absolute => f.write_str("absolute"),
            PositionType::Relative => f.write_str("relative"),
        }
    }
}

impl ValueFor<Position> for PositionType {}
