pub enum FontKerning {
    Auto,
    None,
    Normal,
}
impl std::fmt::Display for FontKerning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FontKerning::Auto => write!(f, "auto"),
            FontKerning::None => write!(f, "none"),
            FontKerning::Normal => write!(f, "normal"),
        }
    }
}
impl crate::ValueFor<FontKerning> for FontKerning {}
impl crate::Attribute for FontKerning {
    const NAME: &'static str = "font-kerning";
}
impl crate::StyleSheet {
    pub fn font_kerning<V: crate::ValueFor<FontKerning>>(mut self, value: V) -> Self {
        self.rules.insert("font-kerning", value.value());
        self
    }
}
