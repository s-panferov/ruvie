pub enum FontVariantPosition {
    Normal,
    Sub,
    Super,
}
impl std::fmt::Display for FontVariantPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FontVariantPosition::Normal => write!(f, "normal"),
            FontVariantPosition::Sub => write!(f, "sub"),
            FontVariantPosition::Super => write!(f, "super"),
        }
    }
}
impl crate::ValueFor<FontVariantPosition> for FontVariantPosition {}
impl crate::Attribute for FontVariantPosition {
    const NAME: &'static str = "font-variant-position";
}
impl crate::StyleSheet {
    pub fn font_variant_position<V: crate::ValueFor<FontVariantPosition>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("font-variant-position", value.value());
        self
    }
}
