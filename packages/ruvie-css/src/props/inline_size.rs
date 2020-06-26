pub enum InlineSize {}
impl crate::Attribute for InlineSize {
    const NAME: &'static str = "inline-size";
}
impl crate::StyleSheet {
    pub fn inline_size<V: crate::ValueFor<InlineSize>>(mut self, value: V) -> Self {
        self.rules.insert("inline-size", value.value());
        self
    }
}
