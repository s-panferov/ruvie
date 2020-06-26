pub enum MarginBlockStart {}
impl crate::Attribute for MarginBlockStart {
    const NAME: &'static str = "margin-block-start";
}
impl crate::StyleSheet {
    pub fn margin_block_start<V: crate::ValueFor<MarginBlockStart>>(mut self, value: V) -> Self {
        self.rules.insert("margin-block-start", value.value());
        self
    }
}
