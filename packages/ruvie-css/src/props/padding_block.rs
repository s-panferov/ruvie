pub enum PaddingBlock {}
impl crate::Attribute for PaddingBlock {
    const NAME: &'static str = "padding-block";
}
impl crate::StyleSheet {
    pub fn padding_block<V: crate::ValueFor<PaddingBlock>>(mut self, value: V) -> Self {
        self.rules.insert("padding-block", value.value());
        self
    }
}
