pub enum OutlineOffset {}
impl crate::Attribute for OutlineOffset {
    const NAME: &'static str = "outline-offset";
}
impl crate::StyleSheet {
    pub fn outline_offset<V: crate::ValueFor<OutlineOffset>>(mut self, value: V) -> Self {
        self.rules.insert("outline-offset", value.value());
        self
    }
}
impl crate::ValueFor<OutlineOffset> for crate::types::length::Length {}
