pub enum MarginRight {
    Auto,
}
impl std::fmt::Display for MarginRight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MarginRight::Auto => write!(f, "auto"),
        }
    }
}
impl crate::ValueFor<MarginRight> for MarginRight {}
impl crate::Attribute for MarginRight {
    const NAME: &'static str = "margin-right";
}
impl crate::StyleSheet {
    pub fn margin_right<V: crate::ValueFor<MarginRight>>(mut self, value: V) -> Self {
        self.rules.insert("margin-right", value.value());
        self
    }
}
impl crate::ValueFor<MarginRight> for crate::types::length::Length {}

impl crate::ValueFor<MarginRight> for crate::types::percentage::Percentage {}
