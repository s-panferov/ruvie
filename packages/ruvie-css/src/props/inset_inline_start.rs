pub enum InsetInlineStart {}
impl crate::Attribute for InsetInlineStart {
    const NAME: &'static str = "inset-inline-start";
}
impl crate::StyleSheet {
    pub fn inset_inline_start<V: crate::ValueFor<InsetInlineStart>>(mut self, value: V) -> Self {
        self.rules.insert("inset-inline-start", value.value());
        self
    }
}
