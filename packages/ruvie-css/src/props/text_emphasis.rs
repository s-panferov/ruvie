pub enum TextEmphasi {}
impl crate::Attribute for TextEmphasi {
    const NAME: &'static str = "text-emphasis";
}
impl crate::StyleSheet {
    pub fn text_emphasis<V: crate::ValueFor<TextEmphasi>>(mut self, value: V) -> Self {
        self.rules.insert("text-emphasis", value.value());
        self
    }
}
