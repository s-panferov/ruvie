pub enum MaskBorderWidth {}
impl crate::Attribute for MaskBorderWidth {
    const NAME: &'static str = "mask-border-width";
}
impl crate::StyleSheet {
    pub fn mask_border_width<V: crate::ValueFor<MaskBorderWidth>>(mut self, value: V) -> Self {
        self.rules.insert("mask-border-width", value.value());
        self
    }
}
