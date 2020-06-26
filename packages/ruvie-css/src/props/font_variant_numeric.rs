pub enum FontVariantNumeric {}
impl crate::Attribute for FontVariantNumeric {
    const NAME: &'static str = "font-variant-numeric";
}
impl crate::StyleSheet {
    pub fn font_variant_numeric<V: crate::ValueFor<FontVariantNumeric>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("font-variant-numeric", value.value());
        self
    }
}
