pub enum OffsetPath {}
impl crate::Attribute for OffsetPath {
    const NAME: &'static str = "offset-path";
}
impl crate::StyleSheet {
    pub fn offset_path<V: crate::ValueFor<OffsetPath>>(mut self, value: V) -> Self {
        self.rules.insert("offset-path", value.value());
        self
    }
}
