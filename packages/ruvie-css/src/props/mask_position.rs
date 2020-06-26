pub enum MaskPosition {}
impl crate::Attribute for MaskPosition {
    const NAME: &'static str = "mask-position";
}
impl crate::StyleSheet {
    pub fn mask_position<V: crate::ValueFor<MaskPosition>>(mut self, value: V) -> Self {
        self.rules.insert("mask-position", value.value());
        self
    }
}
