pub enum WordSpacing {
    Normal,
}
impl std::fmt::Display for WordSpacing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WordSpacing::Normal => write!(f, "normal"),
        }
    }
}
impl crate::ValueFor<WordSpacing> for WordSpacing {}
impl crate::Attribute for WordSpacing {
    const NAME: &'static str = "word-spacing";
}
impl crate::StyleSheet {
    pub fn word_spacing<V: crate::ValueFor<WordSpacing>>(mut self, value: V) -> Self {
        self.rules.insert("word-spacing", value.value());
        self
    }
}

impl crate::ValueFor<WordSpacing> for crate::types::length::Length {}
impl crate::ValueFor<WordSpacing> for crate::types::percentage::Percentage {}
