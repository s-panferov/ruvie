pub enum PaddingInlineStart {}
impl crate::Attribute for PaddingInlineStart {
    const NAME: &'static str = "padding-inline-start";
}
impl crate::StyleSheet {
    pub fn padding_inline_start<V: crate::ValueFor<PaddingInlineStart>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("padding-inline-start", value.value());
        self
    }
}
