pub enum FontVariant {}
impl crate::Attribute for FontVariant {
    const NAME: &'static str = "font-variant";
}
impl crate::StyleSheet {
    pub fn font_variant<V: crate::ValueFor<FontVariant>>(mut self, value: V) -> Self {
        self.rules.insert("font-variant", value.value());
        self
    }
}
