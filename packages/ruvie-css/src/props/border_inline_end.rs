pub enum BorderInlineEnd {}
impl crate::Attribute for BorderInlineEnd {
    const NAME: &'static str = "border-inline-end";
}
impl crate::StyleSheet {
    pub fn border_inline_end<V: crate::ValueFor<BorderInlineEnd>>(mut self, value: V) -> Self {
        self.rules.insert("border-inline-end", value.value());
        self
    }
}
