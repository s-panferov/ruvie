pub enum InsetBlock {}
impl crate::Attribute for InsetBlock {
    const NAME: &'static str = "inset-block";
}
impl crate::StyleSheet {
    pub fn inset_block<V: crate::ValueFor<InsetBlock>>(mut self, value: V) -> Self {
        self.rules.insert("inset-block", value.value());
        self
    }
}
