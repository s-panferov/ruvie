pub enum BorderBlockEndColor {}
impl crate::Attribute for BorderBlockEndColor {
    const NAME: &'static str = "border-block-end-color";
}
impl crate::StyleSheet {
    pub fn border_block_end_color<V: crate::ValueFor<BorderBlockEndColor>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("border-block-end-color", value.value());
        self
    }
}
