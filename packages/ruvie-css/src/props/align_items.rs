pub enum AlignItem {}
impl crate::Attribute for AlignItem {
    const NAME: &'static str = "align-items";
}
impl crate::StyleSheet {
    pub fn align_items<V: crate::ValueFor<AlignItem>>(mut self, value: V) -> Self {
        self.rules.insert("align-items", value.value());
        self
    }
}
