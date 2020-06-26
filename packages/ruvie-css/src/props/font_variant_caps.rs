pub enum FontVariantCap {
    AllPetiteCap,
    AllSmallCap,
    Normal,
    PetiteCap,
    SmallCap,
    TitlingCap,
    Unicase,
}
impl std::fmt::Display for FontVariantCap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FontVariantCap::AllPetiteCap => write!(f, "all-petite-caps"),
            FontVariantCap::AllSmallCap => write!(f, "all-small-caps"),
            FontVariantCap::Normal => write!(f, "normal"),
            FontVariantCap::PetiteCap => write!(f, "petite-caps"),
            FontVariantCap::SmallCap => write!(f, "small-caps"),
            FontVariantCap::TitlingCap => write!(f, "titling-caps"),
            FontVariantCap::Unicase => write!(f, "unicase"),
        }
    }
}
impl crate::ValueFor<FontVariantCap> for FontVariantCap {}
impl crate::Attribute for FontVariantCap {
    const NAME: &'static str = "font-variant-caps";
}
impl crate::StyleSheet {
    pub fn font_variant_caps<V: crate::ValueFor<FontVariantCap>>(mut self, value: V) -> Self {
        self.rules.insert("font-variant-caps", value.value());
        self
    }
}
