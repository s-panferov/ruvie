pub enum TextIndent {}
impl crate::Attribute for TextIndent {
    const NAME: &'static str = "text-indent";
}
impl crate::StyleSheet {
    pub fn text_indent<V: crate::ValueFor<TextIndent>>(mut self, value: V) -> Self {
        self.rules.insert("text-indent", value.value());
        self
    }
}
