pub enum BorderInlineStartWidth {}
impl crate::Attribute for BorderInlineStartWidth {
    const NAME: &'static str = "border-inline-start-width";
}
impl crate::StyleSheet {
    pub fn border_inline_start_width<V: crate::ValueFor<BorderInlineStartWidth>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules
            .insert("border-inline-start-width", value.value());
        self
    }
}
