pub enum Height {
    Auto,
    MaxContent,
    MinContent,
}
impl std::fmt::Display for Height {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Height::Auto => write!(f, "auto"),
            Height::MaxContent => write!(f, "max-content"),
            Height::MinContent => write!(f, "min-content"),
        }
    }
}
impl crate::ValueFor<Height> for Height {}
impl crate::Attribute for Height {
    const NAME: &'static str = "height";
}
impl crate::StyleSheet {
    pub fn height<V: crate::ValueFor<Height>>(mut self, value: V) -> Self {
        self.rules.insert("height", value.value());
        self
    }
}

impl crate::ValueFor<Height> for crate::types::length::Length {}

impl crate::ValueFor<Height> for crate::types::percentage::Percentage {}
