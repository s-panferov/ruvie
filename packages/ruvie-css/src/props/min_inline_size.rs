pub enum MinInlineSize {}
impl crate::Attribute for MinInlineSize {
    const NAME: &'static str = "min-inline-size";
}
impl crate::StyleSheet {
    pub fn min_inline_size<V: crate::ValueFor<MinInlineSize>>(mut self, value: V) -> Self {
        self.rules.insert("min-inline-size", value.value());
        self
    }
}
