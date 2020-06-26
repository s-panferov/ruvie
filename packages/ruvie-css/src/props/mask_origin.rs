pub enum MaskOrigin {}
impl crate::Attribute for MaskOrigin {
    const NAME: &'static str = "mask-origin";
}
impl crate::StyleSheet {
    pub fn mask_origin<V: crate::ValueFor<MaskOrigin>>(mut self, value: V) -> Self {
        self.rules.insert("mask-origin", value.value());
        self
    }
}
