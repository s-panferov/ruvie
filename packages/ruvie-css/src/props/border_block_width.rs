pub enum BorderBlockWidth {}
impl crate::Attribute for BorderBlockWidth {
    const NAME: &'static str = "border-block-width";
}
impl crate::StyleSheet {
    pub fn border_block_width<V: crate::ValueFor<BorderBlockWidth>>(mut self, value: V) -> Self {
        self.rules.insert("border-block-width", value.value());
        self
    }
}
