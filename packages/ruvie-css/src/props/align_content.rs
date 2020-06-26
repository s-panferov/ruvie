pub enum AlignContent {}
impl crate::Attribute for AlignContent {
    const NAME: &'static str = "align-content";
}
impl crate::StyleSheet {
    pub fn align_content<V: crate::ValueFor<AlignContent>>(mut self, value: V) -> Self {
        self.rules.insert("align-content", value.value());
        self
    }
}
