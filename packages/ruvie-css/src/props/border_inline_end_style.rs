pub enum BorderInlineEndStyle {}
impl crate::Attribute for BorderInlineEndStyle {
    const NAME: &'static str = "border-inline-end-style";
}
impl crate::StyleSheet {
    pub fn border_inline_end_style<V: crate::ValueFor<BorderInlineEndStyle>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("border-inline-end-style", value.value());
        self
    }
}
