pub enum FontFeatureSettings {}
impl crate::Attribute for FontFeatureSettings {
    const NAME: &'static str = "font-feature-settings";
}
impl crate::StyleSheet {
    pub fn font_feature_settings<V: crate::ValueFor<FontFeatureSettings>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("font-feature-settings", value.value());
        self
    }
}
