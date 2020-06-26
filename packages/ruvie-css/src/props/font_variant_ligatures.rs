pub enum FontVariantLigatures {}
impl crate::Attribute for FontVariantLigatures {
    const NAME: &'static str = "font-variant-ligatures";
}
impl crate::StyleSheet {
    pub fn font_variant_ligatures<V: crate::ValueFor<FontVariantLigatures>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("font-variant-ligatures", value.value());
        self
    }
}
