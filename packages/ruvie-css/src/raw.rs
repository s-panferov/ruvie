use crate::{Attribute, ValueFor};

use std::{
    fmt::{Display, Formatter},
    ops::Deref,
};

pub struct Raw(pub String);

impl From<String> for Raw {
    fn from(v: String) -> Self {
        Raw(v)
    }
}

impl From<&str> for Raw {
    fn from(v: &str) -> Self {
        Raw(String::from(v))
    }
}

impl Deref for Raw {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Raw {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.0)
    }
}

impl<A> ValueFor<A> for Raw where A: Attribute {}
