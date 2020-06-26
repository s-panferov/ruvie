pub enum BorderInlineStart {}
impl crate::Attribute for BorderInlineStart {
    const NAME: &'static str = "border-inline-start";
}
impl crate::StyleSheet {
    pub fn border_inline_start<V: crate::ValueFor<BorderInlineStart>>(mut self, value: V) -> Self {
        self.rules.insert("border-inline-start", value.value());
        self
    }
}
