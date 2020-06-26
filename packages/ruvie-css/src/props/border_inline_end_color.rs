pub enum BorderInlineEndColor {}
impl crate::Attribute for BorderInlineEndColor {
    const NAME: &'static str = "border-inline-end-color";
}
impl crate::StyleSheet {
    pub fn border_inline_end_color<V: crate::ValueFor<BorderInlineEndColor>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("border-inline-end-color", value.value());
        self
    }
}
