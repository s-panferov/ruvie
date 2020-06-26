pub enum BorderBlockStart {}
impl crate::Attribute for BorderBlockStart {
    const NAME: &'static str = "border-block-start";
}
impl crate::StyleSheet {
    pub fn border_block_start<V: crate::ValueFor<BorderBlockStart>>(mut self, value: V) -> Self {
        self.rules.insert("border-block-start", value.value());
        self
    }
}
