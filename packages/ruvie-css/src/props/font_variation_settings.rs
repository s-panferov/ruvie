pub enum FontVariationSetting {}
impl crate::Attribute for FontVariationSetting {
    const NAME: &'static str = "font-variation-settings";
}
impl crate::StyleSheet {
    pub fn font_variation_settings<V: crate::ValueFor<FontVariationSetting>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("font-variation-settings", value.value());
        self
    }
}
