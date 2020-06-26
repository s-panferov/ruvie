pub enum WordWrap {
    BreakWord,
    Normal,
}
impl std::fmt::Display for WordWrap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WordWrap::BreakWord => write!(f, "break-word"),
            WordWrap::Normal => write!(f, "normal"),
        }
    }
}
impl crate::ValueFor<WordWrap> for WordWrap {}
impl crate::Attribute for WordWrap {
    const NAME: &'static str = "word-wrap";
}
impl crate::StyleSheet {
    pub fn word_wrap<V: crate::ValueFor<WordWrap>>(mut self, value: V) -> Self {
        self.rules.insert("word-wrap", value.value());
        self
    }
}
