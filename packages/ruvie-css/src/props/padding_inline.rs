pub enum PaddingInline {}
impl crate::Attribute for PaddingInline {
    const NAME: &'static str = "padding-inline";
}
impl crate::StyleSheet {
    pub fn padding_inline<V: crate::ValueFor<PaddingInline>>(mut self, value: V) -> Self {
        self.rules.insert("padding-inline", value.value());
        self
    }
}
