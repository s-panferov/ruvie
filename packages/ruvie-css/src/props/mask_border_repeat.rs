pub enum MaskBorderRepeat {}
impl crate::Attribute for MaskBorderRepeat {
    const NAME: &'static str = "mask-border-repeat";
}
impl crate::StyleSheet {
    pub fn mask_border_repeat<V: crate::ValueFor<MaskBorderRepeat>>(mut self, value: V) -> Self {
        self.rules.insert("mask-border-repeat", value.value());
        self
    }
}
