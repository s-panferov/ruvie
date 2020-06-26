pub enum FontFeatureSetting {}
impl crate::Attribute for FontFeatureSetting {
    const NAME: &'static str = "font-feature-settings";
}
impl crate::StyleSheet {
    pub fn font_feature_settings<V: crate::ValueFor<FontFeatureSetting>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("font-feature-settings", value.value());
        self
    }
}
