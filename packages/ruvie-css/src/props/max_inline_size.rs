pub enum MaxInlineSize {}
impl crate::Attribute for MaxInlineSize {
    const NAME: &'static str = "max-inline-size";
}
impl crate::StyleSheet {
    pub fn max_inline_size<V: crate::ValueFor<MaxInlineSize>>(mut self, value: V) -> Self {
        self.rules.insert("max-inline-size", value.value());
        self
    }
}
