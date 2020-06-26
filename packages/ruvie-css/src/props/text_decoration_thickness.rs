pub enum TextDecorationThickness {
    Auto,
    FromFont,
}
impl std::fmt::Display for TextDecorationThickness {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextDecorationThickness::Auto => write!(f, "auto"),
            TextDecorationThickness::FromFont => write!(f, "from-font"),
        }
    }
}
impl crate::ValueFor<TextDecorationThickness> for TextDecorationThickness {}
impl crate::Attribute for TextDecorationThickness {
    const NAME: &'static str = "text-decoration-thickness";
}
impl crate::StyleSheet {
    pub fn text_decoration_thickness<V: crate::ValueFor<TextDecorationThickness>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules
            .insert("text-decoration-thickness", value.value());
        self
    }
}

impl crate::ValueFor<TextDecorationThickness> for crate::types::length::Length {}

impl crate::ValueFor<TextDecorationThickness> for crate::types::percentage::Percentage {}
