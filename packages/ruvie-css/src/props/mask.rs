pub enum Mask {}
impl crate::Attribute for Mask {
    const NAME: &'static str = "mask";
}
impl crate::StyleSheet {
    pub fn mask<V: crate::ValueFor<Mask>>(mut self, value: V) -> Self {
        self.rules.insert("mask", value.value());
        self
    }
}
