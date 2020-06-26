pub enum ScrollMarginInline {}
impl crate::Attribute for ScrollMarginInline {
    const NAME: &'static str = "scroll-margin-inline";
}
impl crate::StyleSheet {
    pub fn scroll_margin_inline<V: crate::ValueFor<ScrollMarginInline>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("scroll-margin-inline", value.value());
        self
    }
}
