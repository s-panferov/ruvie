pub enum BorderBlockEnd {}
impl crate::Attribute for BorderBlockEnd {
    const NAME: &'static str = "border-block-end";
}
impl crate::StyleSheet {
    pub fn border_block_end<V: crate::ValueFor<BorderBlockEnd>>(mut self, value: V) -> Self {
        self.rules.insert("border-block-end", value.value());
        self
    }
}
