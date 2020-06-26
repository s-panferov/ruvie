pub enum LetterSpacing {
    Normal,
}
impl std::fmt::Display for LetterSpacing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LetterSpacing::Normal => write!(f, "normal"),
        }
    }
}
impl crate::ValueFor<LetterSpacing> for LetterSpacing {}
impl crate::Attribute for LetterSpacing {
    const NAME: &'static str = "letter-spacing";
}
impl crate::StyleSheet {
    pub fn letter_spacing<V: crate::ValueFor<LetterSpacing>>(mut self, value: V) -> Self {
        self.rules.insert("letter-spacing", value.value());
        self
    }
}

impl crate::ValueFor<LetterSpacing> for crate::types::length::Length {}
