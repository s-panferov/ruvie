pub enum MaskBorder {}
impl crate::Attribute for MaskBorder {
    const NAME: &'static str = "mask-border";
}
impl crate::StyleSheet {
    pub fn mask_border<V: crate::ValueFor<MaskBorder>>(mut self, value: V) -> Self {
        self.rules.insert("mask-border", value.value());
        self
    }
}
