pub enum MaskSize {}
impl crate::Attribute for MaskSize {
    const NAME: &'static str = "mask-size";
}
impl crate::StyleSheet {
    pub fn mask_size<V: crate::ValueFor<MaskSize>>(mut self, value: V) -> Self {
        self.rules.insert("mask-size", value.value());
        self
    }
}
