pub enum ScrollMarginInlineEnd {}
impl crate::Attribute for ScrollMarginInlineEnd {
    const NAME: &'static str = "scroll-margin-inline-end";
}
impl crate::StyleSheet {
    pub fn scroll_margin_inline_end<V: crate::ValueFor<ScrollMarginInlineEnd>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("scroll-margin-inline-end", value.value());
        self
    }
}
impl crate::ValueFor<ScrollMarginInlineEnd> for crate::types::length::Length {}
