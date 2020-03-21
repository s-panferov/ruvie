mod background;
mod height;
mod left;
mod position;
mod top;
mod width;

use std::fmt::Display;

use crate::rule::Rule;
pub use background::*;
pub use height::*;
pub use left::*;
pub use position::*;
pub use top::*;
pub use width::*;

pub enum Property {
    BackgroundColor(BackgroundColor),
    Height(Height),
    Left(Left),
    Top(Top),
    Position(Position),
    Width(Width),
}

impl Rule for Property {
    fn name(&self) -> &str {
        match self {
            Self::BackgroundColor(v) => v.name(),
            Self::Height(v) => v.name(),
            Self::Left(v) => v.name(),
            Self::Top(v) => v.name(),
            Self::Position(v) => v.name(),
            Self::Width(v) => v.name(),
        }
    }
}

impl Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::BackgroundColor(v) => v.fmt(f),
            Self::Height(v) => v.fmt(f),
            Self::Left(v) => v.fmt(f),
            Self::Top(v) => v.fmt(f),
            Self::Position(v) => v.fmt(f),
            Self::Width(v) => v.fmt(f),
        }
    }
}

impl From<BackgroundColor> for Property {
    fn from(value: BackgroundColor) -> Property {
        Property::BackgroundColor(value)
    }
}

impl From<Height> for Property {
    fn from(value: Height) -> Property {
        Property::Height(value)
    }
}

impl From<Left> for Property {
    fn from(value: Left) -> Property {
        Property::Left(value)
    }
}

impl From<Top> for Property {
    fn from(value: Top) -> Property {
        Property::Top(value)
    }
}

impl From<Position> for Property {
    fn from(value: Position) -> Property {
        Property::Position(value)
    }
}

impl From<Width> for Property {
    fn from(value: Width) -> Property {
        Property::Width(value)
    }
}
