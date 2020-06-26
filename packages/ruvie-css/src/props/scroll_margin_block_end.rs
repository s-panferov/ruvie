pub enum ScrollMarginBlockEnd {}
impl crate::Attribute for ScrollMarginBlockEnd {
    const NAME: &'static str = "scroll-margin-block-end";
}
impl crate::StyleSheet {
    pub fn scroll_margin_block_end<V: crate::ValueFor<ScrollMarginBlockEnd>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("scroll-margin-block-end", value.value());
        self
    }
}
impl crate::ValueFor<ScrollMarginBlockEnd> for crate::types::length::Length {}
