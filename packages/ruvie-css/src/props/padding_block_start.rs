pub enum PaddingBlockStart {}
impl crate::Attribute for PaddingBlockStart {
    const NAME: &'static str = "padding-block-start";
}
impl crate::StyleSheet {
    pub fn padding_block_start<V: crate::ValueFor<PaddingBlockStart>>(mut self, value: V) -> Self {
        self.rules.insert("padding-block-start", value.value());
        self
    }
}
