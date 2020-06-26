pub enum WordBreak {
    BreakAll,
    BreakWord,
    KeepAll,
    Normal,
}
impl std::fmt::Display for WordBreak {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WordBreak::BreakAll => write!(f, "break-all"),
            WordBreak::BreakWord => write!(f, "break-word"),
            WordBreak::KeepAll => write!(f, "keep-all"),
            WordBreak::Normal => write!(f, "normal"),
        }
    }
}
impl crate::ValueFor<WordBreak> for WordBreak {}
impl crate::Attribute for WordBreak {
    const NAME: &'static str = "word-break";
}
impl crate::StyleSheet {
    pub fn word_break<V: crate::ValueFor<WordBreak>>(mut self, value: V) -> Self {
        self.rules.insert("word-break", value.value());
        self
    }
}
