pub enum InsetInlineEnd {}
impl crate::Attribute for InsetInlineEnd {
    const NAME: &'static str = "inset-inline-end";
}
impl crate::StyleSheet {
    pub fn inset_inline_end<V: crate::ValueFor<InsetInlineEnd>>(mut self, value: V) -> Self {
        self.rules.insert("inset-inline-end", value.value());
        self
    }
}
