pub enum BorderBlockStartStyle {}
impl crate::Attribute for BorderBlockStartStyle {
    const NAME: &'static str = "border-block-start-style";
}
impl crate::StyleSheet {
    pub fn border_block_start_style<V: crate::ValueFor<BorderBlockStartStyle>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("border-block-start-style", value.value());
        self
    }
}
