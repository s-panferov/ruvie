pub enum TextEmphasis {}
impl crate::Attribute for TextEmphasis {
    const NAME: &'static str = "text-emphasis";
}
impl crate::StyleSheet {
    pub fn text_emphasis<V: crate::ValueFor<TextEmphasis>>(mut self, value: V) -> Self {
        self.rules.insert("text-emphasis", value.value());
        self
    }
}
