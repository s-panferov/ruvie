pub enum TextEmphasisStyle {}
impl crate::Attribute for TextEmphasisStyle {
    const NAME: &'static str = "text-emphasis-style";
}
impl crate::StyleSheet {
    pub fn text_emphasis_style<V: crate::ValueFor<TextEmphasisStyle>>(mut self, value: V) -> Self {
        self.rules.insert("text-emphasis-style", value.value());
        self
    }
}
