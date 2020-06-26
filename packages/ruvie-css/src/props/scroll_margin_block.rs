pub enum ScrollMarginBlock {}
impl crate::Attribute for ScrollMarginBlock {
    const NAME: &'static str = "scroll-margin-block";
}
impl crate::StyleSheet {
    pub fn scroll_margin_block<V: crate::ValueFor<ScrollMarginBlock>>(mut self, value: V) -> Self {
        self.rules.insert("scroll-margin-block", value.value());
        self
    }
}
