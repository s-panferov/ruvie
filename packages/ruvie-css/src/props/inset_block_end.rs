pub enum InsetBlockEnd {}
impl crate::Attribute for InsetBlockEnd {
    const NAME: &'static str = "inset-block-end";
}
impl crate::StyleSheet {
    pub fn inset_block_end<V: crate::ValueFor<InsetBlockEnd>>(mut self, value: V) -> Self {
        self.rules.insert("inset-block-end", value.value());
        self
    }
}
