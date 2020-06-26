pub enum MarginBottom {
    Auto,
}
impl std::fmt::Display for MarginBottom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MarginBottom::Auto => write!(f, "auto"),
        }
    }
}
impl crate::ValueFor<MarginBottom> for MarginBottom {}
impl crate::Attribute for MarginBottom {
    const NAME: &'static str = "margin-bottom";
}
impl crate::StyleSheet {
    pub fn margin_bottom<V: crate::ValueFor<MarginBottom>>(mut self, value: V) -> Self {
        self.rules.insert("margin-bottom", value.value());
        self
    }
}
impl crate::ValueFor<MarginBottom> for crate::types::length::Length {}

impl crate::ValueFor<MarginBottom> for crate::types::percentage::Percentage {}
