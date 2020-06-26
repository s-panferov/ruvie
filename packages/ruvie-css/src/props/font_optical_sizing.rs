pub enum FontOpticalSizing {
    Auto,
    None,
}
impl std::fmt::Display for FontOpticalSizing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FontOpticalSizing::Auto => write!(f, "auto"),
            FontOpticalSizing::None => write!(f, "none"),
        }
    }
}
impl crate::ValueFor<FontOpticalSizing> for FontOpticalSizing {}
impl crate::Attribute for FontOpticalSizing {
    const NAME: &'static str = "font-optical-sizing";
}
impl crate::StyleSheet {
    pub fn font_optical_sizing<V: crate::ValueFor<FontOpticalSizing>>(mut self, value: V) -> Self {
        self.rules.insert("font-optical-sizing", value.value());
        self
    }
}
