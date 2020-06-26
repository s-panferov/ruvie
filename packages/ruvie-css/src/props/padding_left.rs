pub enum PaddingLeft {}
impl crate::Attribute for PaddingLeft {
    const NAME: &'static str = "padding-left";
}
impl crate::StyleSheet {
    pub fn padding_left<V: crate::ValueFor<PaddingLeft>>(mut self, value: V) -> Self {
        self.rules.insert("padding-left", value.value());
        self
    }
}
impl crate::ValueFor<PaddingLeft> for crate::types::length::Length {}

impl crate::ValueFor<PaddingLeft> for crate::types::percentage::Percentage {}
