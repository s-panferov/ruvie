pub enum MarginTop {
    Auto,
}
impl std::fmt::Display for MarginTop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MarginTop::Auto => write!(f, "auto"),
        }
    }
}
impl crate::ValueFor<MarginTop> for MarginTop {}
impl crate::Attribute for MarginTop {
    const NAME: &'static str = "margin-top";
}
impl crate::StyleSheet {
    pub fn margin_top<V: crate::ValueFor<MarginTop>>(mut self, value: V) -> Self {
        self.rules.insert("margin-top", value.value());
        self
    }
}
impl crate::ValueFor<MarginTop> for crate::types::length::Length {}

impl crate::ValueFor<MarginTop> for crate::types::percentage::Percentage {}
