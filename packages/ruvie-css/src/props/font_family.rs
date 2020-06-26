pub enum FontFamily {}
impl crate::Attribute for FontFamily {
    const NAME: &'static str = "font-family";
}
impl crate::StyleSheet {
    pub fn font_family<V: crate::ValueFor<FontFamily>>(mut self, value: V) -> Self {
        self.rules.insert("font-family", value.value());
        self
    }
}
