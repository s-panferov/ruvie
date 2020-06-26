pub enum Scale {}
impl crate::Attribute for Scale {
    const NAME: &'static str = "scale";
}
impl crate::StyleSheet {
    pub fn scale<V: crate::ValueFor<Scale>>(mut self, value: V) -> Self {
        self.rules.insert("scale", value.value());
        self
    }
}
