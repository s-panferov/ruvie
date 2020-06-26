pub enum FontVariantLigature {}
impl crate::Attribute for FontVariantLigature {
    const NAME: &'static str = "font-variant-ligatures";
}
impl crate::StyleSheet {
    pub fn font_variant_ligatures<V: crate::ValueFor<FontVariantLigature>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("font-variant-ligatures", value.value());
        self
    }
}
