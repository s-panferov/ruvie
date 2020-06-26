pub enum BorderInlineStartStyle {}
impl crate::Attribute for BorderInlineStartStyle {
    const NAME: &'static str = "border-inline-start-style";
}
impl crate::StyleSheet {
    pub fn border_inline_start_style<V: crate::ValueFor<BorderInlineStartStyle>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules
            .insert("border-inline-start-style", value.value());
        self
    }
}
