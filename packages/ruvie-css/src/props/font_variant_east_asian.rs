pub enum FontVariantEastAsian {}
impl crate::Attribute for FontVariantEastAsian {
    const NAME: &'static str = "font-variant-east-asian";
}
impl crate::StyleSheet {
    pub fn font_variant_east_asian<V: crate::ValueFor<FontVariantEastAsian>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("font-variant-east-asian", value.value());
        self
    }
}
