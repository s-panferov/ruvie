pub enum PaddingInlineEnd {}
impl crate::Attribute for PaddingInlineEnd {
    const NAME: &'static str = "padding-inline-end";
}
impl crate::StyleSheet {
    pub fn padding_inline_end<V: crate::ValueFor<PaddingInlineEnd>>(mut self, value: V) -> Self {
        self.rules.insert("padding-inline-end", value.value());
        self
    }
}
