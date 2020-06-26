pub enum BorderBlockStyle {}
impl crate::Attribute for BorderBlockStyle {
    const NAME: &'static str = "border-block-style";
}
impl crate::StyleSheet {
    pub fn border_block_style<V: crate::ValueFor<BorderBlockStyle>>(mut self, value: V) -> Self {
        self.rules.insert("border-block-style", value.value());
        self
    }
}
