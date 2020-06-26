pub enum FontVariationSettings {}
impl crate::Attribute for FontVariationSettings {
    const NAME: &'static str = "font-variation-settings";
}
impl crate::StyleSheet {
    pub fn font_variation_settings<V: crate::ValueFor<FontVariationSettings>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("font-variation-settings", value.value());
        self
    }
}
