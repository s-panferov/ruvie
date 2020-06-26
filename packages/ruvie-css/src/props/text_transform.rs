pub enum TextTransform {
    Capitalize,
    FullSizeKana,
    FullWidth,
    Lowercase,
    None,
    Uppercase,
}
impl std::fmt::Display for TextTransform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextTransform::Capitalize => write!(f, "capitalize"),
            TextTransform::FullSizeKana => write!(f, "full-size-kana"),
            TextTransform::FullWidth => write!(f, "full-width"),
            TextTransform::Lowercase => write!(f, "lowercase"),
            TextTransform::None => write!(f, "none"),
            TextTransform::Uppercase => write!(f, "uppercase"),
        }
    }
}
impl crate::ValueFor<TextTransform> for TextTransform {}
impl crate::Attribute for TextTransform {
    const NAME: &'static str = "text-transform";
}
impl crate::StyleSheet {
    pub fn text_transform<V: crate::ValueFor<TextTransform>>(mut self, value: V) -> Self {
        self.rules.insert("text-transform", value.value());
        self
    }
}
