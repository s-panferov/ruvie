pub enum InsetBlockStart {}
impl crate::Attribute for InsetBlockStart {
    const NAME: &'static str = "inset-block-start";
}
impl crate::StyleSheet {
    pub fn inset_block_start<V: crate::ValueFor<InsetBlockStart>>(mut self, value: V) -> Self {
        self.rules.insert("inset-block-start", value.value());
        self
    }
}
