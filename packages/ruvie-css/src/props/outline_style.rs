pub enum OutlineStyle {
    Auto,
}
impl std::fmt::Display for OutlineStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutlineStyle::Auto => write!(f, "auto"),
        }
    }
}
impl crate::ValueFor<OutlineStyle> for OutlineStyle {}
impl crate::Attribute for OutlineStyle {
    const NAME: &'static str = "outline-style";
}
impl crate::StyleSheet {
    pub fn outline_style<V: crate::ValueFor<OutlineStyle>>(mut self, value: V) -> Self {
        self.rules.insert("outline-style", value.value());
        self
    }
}
