pub enum BorderBlockStartWidth {}
impl crate::Attribute for BorderBlockStartWidth {
    const NAME: &'static str = "border-block-start-width";
}
impl crate::StyleSheet {
    pub fn border_block_start_width<V: crate::ValueFor<BorderBlockStartWidth>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("border-block-start-width", value.value());
        self
    }
}
