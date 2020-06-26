pub enum PaddingRight {}
impl crate::Attribute for PaddingRight {
    const NAME: &'static str = "padding-right";
}
impl crate::StyleSheet {
    pub fn padding_right<V: crate::ValueFor<PaddingRight>>(mut self, value: V) -> Self {
        self.rules.insert("padding-right", value.value());
        self
    }
}
impl crate::ValueFor<PaddingRight> for crate::types::length::Length {}

impl crate::ValueFor<PaddingRight> for crate::types::percentage::Percentage {}
