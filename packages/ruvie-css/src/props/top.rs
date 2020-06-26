pub enum Top {
    Auto,
}
impl std::fmt::Display for Top {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Top::Auto => write!(f, "auto"),
        }
    }
}
impl crate::ValueFor<Top> for Top {}
impl crate::Attribute for Top {
    const NAME: &'static str = "top";
}
impl crate::StyleSheet {
    pub fn top<V: crate::ValueFor<Top>>(mut self, value: V) -> Self {
        self.rules.insert("top", value.value());
        self
    }
}
impl crate::ValueFor<Top> for crate::types::length::Length {}

impl crate::ValueFor<Top> for crate::types::percentage::Percentage {}
