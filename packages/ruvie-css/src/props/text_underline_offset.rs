pub enum TextUnderlineOffset {
    Auto,
}
impl std::fmt::Display for TextUnderlineOffset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextUnderlineOffset::Auto => write!(f, "auto"),
        }
    }
}
impl crate::ValueFor<TextUnderlineOffset> for TextUnderlineOffset {}
impl crate::Attribute for TextUnderlineOffset {
    const NAME: &'static str = "text-underline-offset";
}
impl crate::StyleSheet {
    pub fn text_underline_offset<V: crate::ValueFor<TextUnderlineOffset>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("text-underline-offset", value.value());
        self
    }
}

impl crate::ValueFor<TextUnderlineOffset> for crate::types::length::Length {}

impl crate::ValueFor<TextUnderlineOffset> for crate::types::percentage::Percentage {}
