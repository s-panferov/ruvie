pub enum BorderBlockEndWidth {}
impl crate::Attribute for BorderBlockEndWidth {
    const NAME: &'static str = "border-block-end-width";
}
impl crate::StyleSheet {
    pub fn border_block_end_width<V: crate::ValueFor<BorderBlockEndWidth>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("border-block-end-width", value.value());
        self
    }
}
