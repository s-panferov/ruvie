use std::fmt::{Display, Formatter, Result};

pub struct Ratio(u32, u32);

impl From<(u32, u32)> for Ratio {
    fn from(v: (u32, u32)) -> Self {
        Ratio(v.0, v.1)
    }
}

impl Display for Ratio {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}/{}", self.0, self.1)
    }
}
