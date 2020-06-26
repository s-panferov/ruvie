pub enum MaskRepeat {}
impl crate::Attribute for MaskRepeat {
    const NAME: &'static str = "mask-repeat";
}
impl crate::StyleSheet {
    pub fn mask_repeat<V: crate::ValueFor<MaskRepeat>>(mut self, value: V) -> Self {
        self.rules.insert("mask-repeat", value.value());
        self
    }
}
