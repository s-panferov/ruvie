pub enum MarginBlockEnd {}
impl crate::Attribute for MarginBlockEnd {
    const NAME: &'static str = "margin-block-end";
}
impl crate::StyleSheet {
    pub fn margin_block_end<V: crate::ValueFor<MarginBlockEnd>>(mut self, value: V) -> Self {
        self.rules.insert("margin-block-end", value.value());
        self
    }
}
