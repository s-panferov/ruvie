pub enum MarginBlock {}
impl crate::Attribute for MarginBlock {
    const NAME: &'static str = "margin-block";
}
impl crate::StyleSheet {
    pub fn margin_block<V: crate::ValueFor<MarginBlock>>(mut self, value: V) -> Self {
        self.rules.insert("margin-block", value.value());
        self
    }
}
