pub enum MaskImage {}
impl crate::Attribute for MaskImage {
    const NAME: &'static str = "mask-image";
}
impl crate::StyleSheet {
    pub fn mask_image<V: crate::ValueFor<MaskImage>>(mut self, value: V) -> Self {
        self.rules.insert("mask-image", value.value());
        self
    }
}
