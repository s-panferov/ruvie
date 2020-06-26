use std::fmt::Display;

#[derive(Hash)]
pub enum Color {
    Rgba(u8, u8, u8, u8),
    Rgb(u8, u8, u8),
    Basic(BasicColor),
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::Basic(v) => v.fmt(f),
            Self::Rgb(r, g, b) => write!(f, "rgb({},{},{})", r, g, b),
            Self::Rgba(r, g, b, a) => write!(f, "rgba({},{},{},{})", r, g, b, a),
        }
    }
}

#[derive(Hash)]
pub enum BasicColor {
    Black,
    Silver,
    Gray,
    White,
    Maroon,
    Red,
    Purple,
    Fuchsia,
    Green,
    Lime,
    Olive,
    Yellow,
    Navy,
    Blue,
    Teal,
    Aqua,
}

impl From<BasicColor> for Color {
    fn from(value: BasicColor) -> Color {
        Color::Basic(value)
    }
}

impl Display for BasicColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::Black => f.write_str("black"),
            Self::Silver => f.write_str("silver"),
            Self::Gray => f.write_str("gray"),
            Self::White => f.write_str("white"),
            Self::Maroon => f.write_str("maroon"),
            Self::Red => f.write_str("red"),
            Self::Purple => f.write_str("purple"),
            Self::Fuchsia => f.write_str("fuchsia"),
            Self::Green => f.write_str("green"),
            Self::Lime => f.write_str("lime"),
            Self::Olive => f.write_str("olive"),
            Self::Yellow => f.write_str("yellow"),
            Self::Navy => f.write_str("navy"),
            Self::Blue => f.write_str("blue"),
            Self::Teal => f.write_str("teal"),
            Self::Aqua => f.write_str("aqua"),
        }
    }
}
