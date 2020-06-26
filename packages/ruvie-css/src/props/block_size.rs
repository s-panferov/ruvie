pub enum BlockSize {}
impl crate::Attribute for BlockSize {
    const NAME: &'static str = "block-size";
}
impl crate::StyleSheet {
    pub fn block_size<V: crate::ValueFor<BlockSize>>(mut self, value: V) -> Self {
        self.rules.insert("block-size", value.value());
        self
    }
}
