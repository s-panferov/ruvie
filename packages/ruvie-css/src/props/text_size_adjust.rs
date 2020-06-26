pub enum TextSizeAdjust {
    Auto,
    None,
}
impl std::fmt::Display for TextSizeAdjust {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextSizeAdjust::Auto => write!(f, "auto"),
            TextSizeAdjust::None => write!(f, "none"),
        }
    }
}
impl crate::ValueFor<TextSizeAdjust> for TextSizeAdjust {}
impl crate::Attribute for TextSizeAdjust {
    const NAME: &'static str = "text-size-adjust";
}
impl crate::StyleSheet {
    pub fn text_size_adjust<V: crate::ValueFor<TextSizeAdjust>>(mut self, value: V) -> Self {
        self.rules.insert("text-size-adjust", value.value());
        self
    }
}

impl crate::ValueFor<TextSizeAdjust> for crate::types::percentage::Percentage {}
