pub enum MaskComposite {}
impl crate::Attribute for MaskComposite {
    const NAME: &'static str = "mask-composite";
}
impl crate::StyleSheet {
    pub fn mask_composite<V: crate::ValueFor<MaskComposite>>(mut self, value: V) -> Self {
        self.rules.insert("mask-composite", value.value());
        self
    }
}
