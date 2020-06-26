pub enum FontVariantAlternates {}
impl crate::Attribute for FontVariantAlternates {
    const NAME: &'static str = "font-variant-alternates";
}
impl crate::StyleSheet {
    pub fn font_variant_alternates<V: crate::ValueFor<FontVariantAlternates>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("font-variant-alternates", value.value());
        self
    }
}
