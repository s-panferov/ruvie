pub enum TextEmphasisPosition {}
impl crate::Attribute for TextEmphasisPosition {
    const NAME: &'static str = "text-emphasis-position";
}
impl crate::StyleSheet {
    pub fn text_emphasis_position<V: crate::ValueFor<TextEmphasisPosition>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("text-emphasis-position", value.value());
        self
    }
}
