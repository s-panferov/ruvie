pub enum MaskMode {}
impl crate::Attribute for MaskMode {
    const NAME: &'static str = "mask-mode";
}
impl crate::StyleSheet {
    pub fn mask_mode<V: crate::ValueFor<MaskMode>>(mut self, value: V) -> Self {
        self.rules.insert("mask-mode", value.value());
        self
    }
}
