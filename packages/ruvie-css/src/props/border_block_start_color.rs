pub enum BorderBlockStartColor {}
impl crate::Attribute for BorderBlockStartColor {
    const NAME: &'static str = "border-block-start-color";
}
impl crate::StyleSheet {
    pub fn border_block_start_color<V: crate::ValueFor<BorderBlockStartColor>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("border-block-start-color", value.value());
        self
    }
}
