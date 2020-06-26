pub enum BorderInlineStartColor {}
impl crate::Attribute for BorderInlineStartColor {
    const NAME: &'static str = "border-inline-start-color";
}
impl crate::StyleSheet {
    pub fn border_inline_start_color<V: crate::ValueFor<BorderInlineStartColor>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules
            .insert("border-inline-start-color", value.value());
        self
    }
}
