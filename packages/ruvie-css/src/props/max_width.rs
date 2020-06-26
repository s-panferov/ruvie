pub enum MaxWidth {
    Auto,
    MaxContent,
    MinContent,
}
impl std::fmt::Display for MaxWidth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MaxWidth::Auto => write!(f, "auto"),
            MaxWidth::MaxContent => write!(f, "max-content"),
            MaxWidth::MinContent => write!(f, "min-content"),
        }
    }
}
impl crate::ValueFor<MaxWidth> for MaxWidth {}
impl crate::Attribute for MaxWidth {
    const NAME: &'static str = "max-width";
}
impl crate::StyleSheet {
    pub fn max_width<V: crate::ValueFor<MaxWidth>>(mut self, value: V) -> Self {
        self.rules.insert("max-width", value.value());
        self
    }
}

impl crate::ValueFor<MaxWidth> for crate::types::length::Length {}

impl crate::ValueFor<MaxWidth> for crate::types::percentage::Percentage {}
