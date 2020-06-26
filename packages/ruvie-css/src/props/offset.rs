pub enum Offset {}
impl crate::Attribute for Offset {
    const NAME: &'static str = "offset";
}
impl crate::StyleSheet {
    pub fn offset<V: crate::ValueFor<Offset>>(mut self, value: V) -> Self {
        self.rules.insert("offset", value.value());
        self
    }
}
