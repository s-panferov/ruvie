pub enum ScrollMarginBlockStart {}
impl crate::Attribute for ScrollMarginBlockStart {
    const NAME: &'static str = "scroll-margin-block-start";
}
impl crate::StyleSheet {
    pub fn scroll_margin_block_start<V: crate::ValueFor<ScrollMarginBlockStart>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules
            .insert("scroll-margin-block-start", value.value());
        self
    }
}
impl crate::ValueFor<ScrollMarginBlockStart> for crate::types::length::Length {}
