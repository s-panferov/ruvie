pub enum Right {
    Auto,
}
impl std::fmt::Display for Right {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Right::Auto => write!(f, "auto"),
        }
    }
}
impl crate::ValueFor<Right> for Right {}
impl crate::Attribute for Right {
    const NAME: &'static str = "right";
}
impl crate::StyleSheet {
    pub fn right<V: crate::ValueFor<Right>>(mut self, value: V) -> Self {
        self.rules.insert("right", value.value());
        self
    }
}
impl crate::ValueFor<Right> for crate::types::length::Length {}

impl crate::ValueFor<Right> for crate::types::percentage::Percentage {}
