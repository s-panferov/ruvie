pub enum InsetInline {}
impl crate::Attribute for InsetInline {
    const NAME: &'static str = "inset-inline";
}
impl crate::StyleSheet {
    pub fn inset_inline<V: crate::ValueFor<InsetInline>>(mut self, value: V) -> Self {
        self.rules.insert("inset-inline", value.value());
        self
    }
}
