pub enum ScrollPaddingBlock {}
impl crate::Attribute for ScrollPaddingBlock {
    const NAME: &'static str = "scroll-padding-block";
}
impl crate::StyleSheet {
    pub fn scroll_padding_block<V: crate::ValueFor<ScrollPaddingBlock>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("scroll-padding-block", value.value());
        self
    }
}
