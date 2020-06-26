pub enum BorderInline {}
impl crate::Attribute for BorderInline {
    const NAME: &'static str = "border-inline";
}
impl crate::StyleSheet {
    pub fn border_inline<V: crate::ValueFor<BorderInline>>(mut self, value: V) -> Self {
        self.rules.insert("border-inline", value.value());
        self
    }
}
