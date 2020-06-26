pub enum Bottom {
    Auto,
}
impl std::fmt::Display for Bottom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Bottom::Auto => write!(f, "auto"),
        }
    }
}
impl crate::ValueFor<Bottom> for Bottom {}
impl crate::Attribute for Bottom {
    const NAME: &'static str = "bottom";
}
impl crate::StyleSheet {
    pub fn bottom<V: crate::ValueFor<Bottom>>(mut self, value: V) -> Self {
        self.rules.insert("bottom", value.value());
        self
    }
}
impl crate::ValueFor<Bottom> for crate::types::length::Length {}

impl crate::ValueFor<Bottom> for crate::types::percentage::Percentage {}
