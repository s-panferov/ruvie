pub enum PaddingBottom {}
impl crate::Attribute for PaddingBottom {
    const NAME: &'static str = "padding-bottom";
}
impl crate::StyleSheet {
    pub fn padding_bottom<V: crate::ValueFor<PaddingBottom>>(mut self, value: V) -> Self {
        self.rules.insert("padding-bottom", value.value());
        self
    }
}
impl crate::ValueFor<PaddingBottom> for crate::types::length::Length {}

impl crate::ValueFor<PaddingBottom> for crate::types::percentage::Percentage {}
