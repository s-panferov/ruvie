pub enum TextUnderlinePosition {}
impl crate::Attribute for TextUnderlinePosition {
    const NAME: &'static str = "text-underline-position";
}
impl crate::StyleSheet {
    pub fn text_underline_position<V: crate::ValueFor<TextUnderlinePosition>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("text-underline-position", value.value());
        self
    }
}
