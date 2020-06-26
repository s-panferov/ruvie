pub enum MaskBorderlouse {}
impl crate::Attribute for MaskBorderlouse {
    const NAME: &'static str = "mask-border-slice";
}
impl crate::StyleSheet {
    pub fn mask_border_slice<V: crate::ValueFor<MaskBorderlouse>>(mut self, value: V) -> Self {
        self.rules.insert("mask-border-slice", value.value());
        self
    }
}
