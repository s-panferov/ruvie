pub enum MaskBorderSlice {}
impl crate::Attribute for MaskBorderSlice {
    const NAME: &'static str = "mask-border-slice";
}
impl crate::StyleSheet {
    pub fn mask_border_slice<V: crate::ValueFor<MaskBorderSlice>>(mut self, value: V) -> Self {
        self.rules.insert("mask-border-slice", value.value());
        self
    }
}
