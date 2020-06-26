pub enum MarginLeft {
    Auto,
}
impl std::fmt::Display for MarginLeft {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MarginLeft::Auto => write!(f, "auto"),
        }
    }
}
impl crate::ValueFor<MarginLeft> for MarginLeft {}
impl crate::Attribute for MarginLeft {
    const NAME: &'static str = "margin-left";
}
impl crate::StyleSheet {
    pub fn margin_left<V: crate::ValueFor<MarginLeft>>(mut self, value: V) -> Self {
        self.rules.insert("margin-left", value.value());
        self
    }
}
impl crate::ValueFor<MarginLeft> for crate::types::length::Length {}

impl crate::ValueFor<MarginLeft> for crate::types::percentage::Percentage {}
