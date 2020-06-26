pub enum BorderBlockEndStyle {}
impl crate::Attribute for BorderBlockEndStyle {
    const NAME: &'static str = "border-block-end-style";
}
impl crate::StyleSheet {
    pub fn border_block_end_style<V: crate::ValueFor<BorderBlockEndStyle>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("border-block-end-style", value.value());
        self
    }
}
