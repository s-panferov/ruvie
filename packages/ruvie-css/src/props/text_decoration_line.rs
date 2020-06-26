pub enum TextDecorationLine {}
impl crate::Attribute for TextDecorationLine {
    const NAME: &'static str = "text-decoration-line";
}
impl crate::StyleSheet {
    pub fn text_decoration_line<V: crate::ValueFor<TextDecorationLine>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("text-decoration-line", value.value());
        self
    }
}
