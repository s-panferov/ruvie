pub enum PaddingBlockEnd {}
impl crate::Attribute for PaddingBlockEnd {
    const NAME: &'static str = "padding-block-end";
}
impl crate::StyleSheet {
    pub fn padding_block_end<V: crate::ValueFor<PaddingBlockEnd>>(mut self, value: V) -> Self {
        self.rules.insert("padding-block-end", value.value());
        self
    }
}
