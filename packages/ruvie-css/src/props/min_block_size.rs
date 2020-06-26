pub enum MinBlockSize {}
impl crate::Attribute for MinBlockSize {
    const NAME: &'static str = "min-block-size";
}
impl crate::StyleSheet {
    pub fn min_block_size<V: crate::ValueFor<MinBlockSize>>(mut self, value: V) -> Self {
        self.rules.insert("min-block-size", value.value());
        self
    }
}
