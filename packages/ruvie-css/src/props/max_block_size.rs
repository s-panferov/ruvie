pub enum MaxBlockSize {}
impl crate::Attribute for MaxBlockSize {
    const NAME: &'static str = "max-block-size";
}
impl crate::StyleSheet {
    pub fn max_block_size<V: crate::ValueFor<MaxBlockSize>>(mut self, value: V) -> Self {
        self.rules.insert("max-block-size", value.value());
        self
    }
}
