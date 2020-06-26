pub enum TextDecorationSkip {}
impl crate::Attribute for TextDecorationSkip {
    const NAME: &'static str = "text-decoration-skip";
}
impl crate::StyleSheet {
    pub fn text_decoration_skip<V: crate::ValueFor<TextDecorationSkip>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("text-decoration-skip", value.value());
        self
    }
}
