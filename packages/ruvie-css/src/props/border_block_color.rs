pub enum BorderBlockColor {}
impl crate::Attribute for BorderBlockColor {
    const NAME: &'static str = "border-block-color";
}
impl crate::StyleSheet {
    pub fn border_block_color<V: crate::ValueFor<BorderBlockColor>>(mut self, value: V) -> Self {
        self.rules.insert("border-block-color", value.value());
        self
    }
}
