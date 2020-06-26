pub enum FontVariantCaps {
    AllPetiteCaps,
    AllSmallCaps,
    Normal,
    PetiteCaps,
    SmallCaps,
    TitlingCaps,
    Unicase,
}
impl std::fmt::Display for FontVariantCaps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FontVariantCaps::AllPetiteCaps => write!(f, "all-petite-caps"),
            FontVariantCaps::AllSmallCaps => write!(f, "all-small-caps"),
            FontVariantCaps::Normal => write!(f, "normal"),
            FontVariantCaps::PetiteCaps => write!(f, "petite-caps"),
            FontVariantCaps::SmallCaps => write!(f, "small-caps"),
            FontVariantCaps::TitlingCaps => write!(f, "titling-caps"),
            FontVariantCaps::Unicase => write!(f, "unicase"),
        }
    }
}
impl crate::ValueFor<FontVariantCaps> for FontVariantCaps {}
impl crate::Attribute for FontVariantCaps {
    const NAME: &'static str = "font-variant-caps";
}
impl crate::StyleSheet {
    pub fn font_variant_caps<V: crate::ValueFor<FontVariantCaps>>(mut self, value: V) -> Self {
        self.rules.insert("font-variant-caps", value.value());
        self
    }
}
