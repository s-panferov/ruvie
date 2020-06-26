pub enum BorderInlineColor {}
impl crate::Attribute for BorderInlineColor {
    const NAME: &'static str = "border-inline-color";
}
impl crate::StyleSheet {
    pub fn border_inline_color<V: crate::ValueFor<BorderInlineColor>>(mut self, value: V) -> Self {
        self.rules.insert("border-inline-color", value.value());
        self
    }
}
