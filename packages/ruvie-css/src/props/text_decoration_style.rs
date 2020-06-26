pub enum TextDecorationStyle {
    Dashed,
    Dotted,
    Double,
    Solid,
    Wavy,
}
impl std::fmt::Display for TextDecorationStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextDecorationStyle::Dashed => write!(f, "dashed"),
            TextDecorationStyle::Dotted => write!(f, "dotted"),
            TextDecorationStyle::Double => write!(f, "double"),
            TextDecorationStyle::Solid => write!(f, "solid"),
            TextDecorationStyle::Wavy => write!(f, "wavy"),
        }
    }
}
impl crate::ValueFor<TextDecorationStyle> for TextDecorationStyle {}
impl crate::Attribute for TextDecorationStyle {
    const NAME: &'static str = "text-decoration-style";
}
impl crate::StyleSheet {
    pub fn text_decoration_style<V: crate::ValueFor<TextDecorationStyle>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("text-decoration-style", value.value());
        self
    }
}
