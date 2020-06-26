pub enum TextDecorationSkipInk {
    All,
    Auto,
    None,
}
impl std::fmt::Display for TextDecorationSkipInk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextDecorationSkipInk::All => write!(f, "all"),
            TextDecorationSkipInk::Auto => write!(f, "auto"),
            TextDecorationSkipInk::None => write!(f, "none"),
        }
    }
}
impl crate::ValueFor<TextDecorationSkipInk> for TextDecorationSkipInk {}
impl crate::Attribute for TextDecorationSkipInk {
    const NAME: &'static str = "text-decoration-skip-ink";
}
impl crate::StyleSheet {
    pub fn text_decoration_skip_ink<V: crate::ValueFor<TextDecorationSkipInk>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("text-decoration-skip-ink", value.value());
        self
    }
}
