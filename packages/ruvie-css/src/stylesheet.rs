use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::{collections::BTreeMap, hash::Hasher};

use crate::rule::{Attribute, ValueFor};

#[derive(Clone)]
pub struct StyleSheet {
    name: Option<&'static str>,
    pub(crate) rules: BTreeMap<&'static str, String>,
}

impl StyleSheet {
    pub fn new() -> StyleSheet {
        StyleSheet {
            name: None,
            rules: BTreeMap::new(),
        }
    }

    pub fn with_name(mut self, name: &'static str) -> Self {
        self.name = Some(name);
        self
    }

    pub fn add<A: Attribute, V: ValueFor<A>>(mut self, value: V) -> Self {
        self.rules.insert(A::NAME, value.value());
        self
    }
}

impl Debug for StyleSheet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        Display::fmt(self, f)
    }
}

impl Display for StyleSheet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        for (k, v) in &self.rules {
            write!(f, "{}: {};", k, v)?
        }

        Ok(())
    }
}

impl Hash for StyleSheet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_string().hash(state)
    }
}
