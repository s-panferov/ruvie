pub enum Left {
    Auto,
}
impl std::fmt::Display for Left {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Left::Auto => write!(f, "auto"),
        }
    }
}
impl crate::ValueFor<Left> for Left {}
impl crate::Attribute for Left {
    const NAME: &'static str = "left";
}
impl crate::StyleSheet {
    pub fn left<V: crate::ValueFor<Left>>(mut self, value: V) -> Self {
        self.rules.insert("left", value.value());
        self
    }
}
impl crate::ValueFor<Left> for crate::types::length::Length {}

impl crate::ValueFor<Left> for crate::types::percentage::Percentage {}
