pub enum FontStretch {}
impl crate::Attribute for FontStretch {
    const NAME: &'static str = "font-stretch";
}
impl crate::StyleSheet {
    pub fn font_stretch<V: crate::ValueFor<FontStretch>>(mut self, value: V) -> Self {
        self.rules.insert("font-stretch", value.value());
        self
    }
}
