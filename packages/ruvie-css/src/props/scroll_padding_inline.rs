pub enum ScrollPaddingInline {}
impl crate::Attribute for ScrollPaddingInline {
    const NAME: &'static str = "scroll-padding-inline";
}
impl crate::StyleSheet {
    pub fn scroll_padding_inline<V: crate::ValueFor<ScrollPaddingInline>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("scroll-padding-inline", value.value());
        self
    }
}
