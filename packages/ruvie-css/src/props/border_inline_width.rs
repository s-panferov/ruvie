pub enum BorderInlineWidth {}
impl crate::Attribute for BorderInlineWidth {
    const NAME: &'static str = "border-inline-width";
}
impl crate::StyleSheet {
    pub fn border_inline_width<V: crate::ValueFor<BorderInlineWidth>>(mut self, value: V) -> Self {
        self.rules.insert("border-inline-width", value.value());
        self
    }
}
