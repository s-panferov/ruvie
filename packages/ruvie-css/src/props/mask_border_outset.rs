pub enum MaskBorderOutset {}
impl crate::Attribute for MaskBorderOutset {
    const NAME: &'static str = "mask-border-outset";
}
impl crate::StyleSheet {
    pub fn mask_border_outset<V: crate::ValueFor<MaskBorderOutset>>(mut self, value: V) -> Self {
        self.rules.insert("mask-border-outset", value.value());
        self
    }
}
