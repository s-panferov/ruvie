pub enum FontVariantAlternate {}
impl crate::Attribute for FontVariantAlternate {
    const NAME: &'static str = "font-variant-alternates";
}
impl crate::StyleSheet {
    pub fn font_variant_alternates<V: crate::ValueFor<FontVariantAlternate>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("font-variant-alternates", value.value());
        self
    }
}
