pub enum FontLanguageOverride {
    Normal,
}
impl std::fmt::Display for FontLanguageOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FontLanguageOverride::Normal => write!(f, "normal"),
        }
    }
}
impl crate::ValueFor<FontLanguageOverride> for FontLanguageOverride {}
impl crate::Attribute for FontLanguageOverride {
    const NAME: &'static str = "font-language-override";
}
impl crate::StyleSheet {
    pub fn font_language_override<V: crate::ValueFor<FontLanguageOverride>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("font-language-override", value.value());
        self
    }
}
