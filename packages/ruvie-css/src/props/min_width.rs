pub enum MinWidth {
    Auto,
    MaxContent,
    MinContent,
}
impl std::fmt::Display for MinWidth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MinWidth::Auto => write!(f, "auto"),
            MinWidth::MaxContent => write!(f, "max-content"),
            MinWidth::MinContent => write!(f, "min-content"),
        }
    }
}
impl crate::ValueFor<MinWidth> for MinWidth {}
impl crate::Attribute for MinWidth {
    const NAME: &'static str = "min-width";
}
impl crate::StyleSheet {
    pub fn min_width<V: crate::ValueFor<MinWidth>>(mut self, value: V) -> Self {
        self.rules.insert("min-width", value.value());
        self
    }
}

impl crate::ValueFor<MinWidth> for crate::types::length::Length {}

impl crate::ValueFor<MinWidth> for crate::types::percentage::Percentage {}
