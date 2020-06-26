pub enum AlignItems {}
impl crate::Attribute for AlignItems {
    const NAME: &'static str = "align-items";
}
impl crate::StyleSheet {
    pub fn align_items<V: crate::ValueFor<AlignItems>>(mut self, value: V) -> Self {
        self.rules.insert("align-items", value.value());
        self
    }
}
