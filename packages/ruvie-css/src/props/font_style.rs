pub enum FontStyle {}
impl crate::Attribute for FontStyle {
    const NAME: &'static str = "font-style";
}
impl crate::StyleSheet {
    pub fn font_style<V: crate::ValueFor<FontStyle>>(mut self, value: V) -> Self {
        self.rules.insert("font-style", value.value());
        self
    }
}
