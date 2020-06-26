pub enum TextOverflow {}
impl crate::Attribute for TextOverflow {
    const NAME: &'static str = "text-overflow";
}
impl crate::StyleSheet {
    pub fn text_overflow<V: crate::ValueFor<TextOverflow>>(mut self, value: V) -> Self {
        self.rules.insert("text-overflow", value.value());
        self
    }
}
