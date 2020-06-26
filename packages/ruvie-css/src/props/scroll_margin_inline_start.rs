pub enum ScrollMarginInlineStart {}
impl crate::Attribute for ScrollMarginInlineStart {
    const NAME: &'static str = "scroll-margin-inline-start";
}
impl crate::StyleSheet {
    pub fn scroll_margin_inline_start<V: crate::ValueFor<ScrollMarginInlineStart>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules
            .insert("scroll-margin-inline-start", value.value());
        self
    }
}
impl crate::ValueFor<ScrollMarginInlineStart> for crate::types::length::Length {}
