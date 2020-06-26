pub enum TextJustify {
    Auto,
    InterCharacter,
    InterWord,
    None,
}
impl std::fmt::Display for TextJustify {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextJustify::Auto => write!(f, "auto"),
            TextJustify::InterCharacter => write!(f, "inter-character"),
            TextJustify::InterWord => write!(f, "inter-word"),
            TextJustify::None => write!(f, "none"),
        }
    }
}
impl crate::ValueFor<TextJustify> for TextJustify {}
impl crate::Attribute for TextJustify {
    const NAME: &'static str = "text-justify";
}
impl crate::StyleSheet {
    pub fn text_justify<V: crate::ValueFor<TextJustify>>(mut self, value: V) -> Self {
        self.rules.insert("text-justify", value.value());
        self
    }
}
