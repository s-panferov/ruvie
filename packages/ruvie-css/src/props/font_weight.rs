pub enum FontWeight {
    Bolder,
    Lighter,
}
impl std::fmt::Display for FontWeight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FontWeight::Bolder => write!(f, "bolder"),
            FontWeight::Lighter => write!(f, "lighter"),
        }
    }
}
impl crate::ValueFor<FontWeight> for FontWeight {}
impl crate::Attribute for FontWeight {
    const NAME: &'static str = "font-weight";
}
impl crate::StyleSheet {
    pub fn font_weight<V: crate::ValueFor<FontWeight>>(mut self, value: V) -> Self {
        self.rules.insert("font-weight", value.value());
        self
    }
}
