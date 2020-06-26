pub enum BorderInlineStyle {}
impl crate::Attribute for BorderInlineStyle {
    const NAME: &'static str = "border-inline-style";
}
impl crate::StyleSheet {
    pub fn border_inline_style<V: crate::ValueFor<BorderInlineStyle>>(mut self, value: V) -> Self {
        self.rules.insert("border-inline-style", value.value());
        self
    }
}
