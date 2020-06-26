pub enum BorderInlineEndWidth {}
impl crate::Attribute for BorderInlineEndWidth {
    const NAME: &'static str = "border-inline-end-width";
}
impl crate::StyleSheet {
    pub fn border_inline_end_width<V: crate::ValueFor<BorderInlineEndWidth>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("border-inline-end-width", value.value());
        self
    }
}
