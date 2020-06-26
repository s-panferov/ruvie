pub enum PaddingTop {}
impl crate::Attribute for PaddingTop {
    const NAME: &'static str = "padding-top";
}
impl crate::StyleSheet {
    pub fn padding_top<V: crate::ValueFor<PaddingTop>>(mut self, value: V) -> Self {
        self.rules.insert("padding-top", value.value());
        self
    }
}
impl crate::ValueFor<PaddingTop> for crate::types::length::Length {}

impl crate::ValueFor<PaddingTop> for crate::types::percentage::Percentage {}
