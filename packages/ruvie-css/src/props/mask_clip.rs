pub enum MaskClip {}
impl crate::Attribute for MaskClip {
    const NAME: &'static str = "mask-clip";
}
impl crate::StyleSheet {
    pub fn mask_clip<V: crate::ValueFor<MaskClip>>(mut self, value: V) -> Self {
        self.rules.insert("mask-clip", value.value());
        self
    }
}
